use async_std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use async_std::task::JoinHandle;
use async_std::net::{TcpStream, ToSocketAddrs, TcpListener};
use futures::{AsyncWriteExt, AsyncReadExt, StreamExt};
use web2core::protoc::{ExecuteInfo, OpResult};
use crate::pipe_tasks::PipeTasks;

pub struct TaskHandler {
    pub inner: Arc<Mutex<HashMap<String, (TcpStream, JoinHandle<()>)>>>,
}

impl TaskHandler {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub async fn start<A: ToSocketAddrs>(&mut self, addrs: A) -> std::io::Result<()> {
        let listener = TcpListener::bind(addrs).await?;
        let mut buf = [0; 1024];

        while let Some(stream) = listener.incoming().next().await {
            let mut socket = stream?;
            let len = socket.read(&mut buf).await?;
            if len == 0 { continue; }

            let token = match core::str::from_utf8(&buf[..len]) {
                Ok(t) => t.trim(),
                Err(_) => continue,
            };

            if self.contains(token) {
                let mut s = self.get_socket(token).unwrap();
                s.close().await.unwrap();
            }

            self.tasks_run(token, socket);
        }
        Ok(())
    }

    pub fn insert(&mut self, token: &str,
                  socket: TcpStream,
                  handle_task: JoinHandle<()>,
    ) {
        self.inner.lock().unwrap().insert(token.into(), (socket, handle_task));
    }

    pub fn get_socket(&mut self, token: &str) -> Option<TcpStream> {
        let handler = self.inner.lock().unwrap().remove(token);
        match handler {
            None => None,
            Some((s, h)) => {
                let socket = s.clone();
                self.insert(token, s, h);
                Some(socket)
            }
        }
    }

    pub async fn execute(&mut self, info: ExecuteInfo) -> OpResult {
        let handler = self.inner.lock().unwrap().remove(&info.get_token());
        match handler {
            None => OpResult::DeviceOffline,
            Some((mut s, h)) => {
                s.write(&info.get_command().as_bytes()).await.unwrap();
                self.inner.lock().unwrap().insert(info.get_token(), (s, h));
                OpResult::Ok
            }
        }
    }

    pub fn contains(&self, token: &str) -> bool {
        self.inner.lock().unwrap().contains_key(token)
    }

    pub fn tasks_run(&mut self, token: &str, socket: TcpStream) {
        let _socket = socket.clone();
        let mut tasks = PipeTasks::new(token, _socket);
        if tasks.is_empty() { return; }

        let handle = async_std::task::spawn(async move {
            loop {
                let fs = tasks.get_all_future().await;
                if fs.is_empty() { break; }
                futures::future::join_all(fs).await;
            }
        });

        let token = token.to_string();
        let token_clone = token.clone();
        let mut _socket = socket.clone();

        self.insert(&token_clone, socket, handle);
    }

    pub async fn reload(&mut self, token: &str) -> OpResult {
        let handler = self.inner.lock().unwrap().remove(token);
        match handler {
            None => OpResult::DeviceOffline,
            Some((s, h)) => {
                h.cancel().await;
                println!("reload done");
                self.tasks_run(token, s);
                OpResult::Ok
            }
        }
    }
}

impl Clone for TaskHandler {
    fn clone(&self) -> Self {
        let inner = self.inner.clone();
        Self {
            inner,
        }
    }
}