use async_std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
use async_std::task::JoinHandle;
use async_std::net::TcpStream;
use futures::{AsyncReadExt, AsyncWriteExt};
use web2core::protoc::ExecuteInfo;

pub struct Handler {
    inner: Arc<Mutex<HashMap<String, (TcpStream, JoinHandle<()>)>>>
}

impl Handler {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn insert(&mut self, token: &str, socket: TcpStream, handle: JoinHandle<()>) {
        self.inner.lock().unwrap().insert(token.into(), (socket, handle));
    }

    pub async fn cancel_guard(&mut self, token: &str) {
        let mut buf = [];
        let handler = self.inner.lock().unwrap().remove(token);
        if handler.is_none() { return; }
        let (mut s, h) = handler.unwrap();
        s.read(&mut buf).await.unwrap();
        h.cancel().await;
    }

    pub async fn execute(&mut self, info: ExecuteInfo) {
        let handler = self.inner.lock().unwrap().remove(&info.get_token());
        if handler.is_none() { return; }
        let (mut s, h) = handler.unwrap();
        s.write(&info.get_command().as_bytes()).await.unwrap();
        self.inner.lock().unwrap().insert(info.get_token(), (s, h)).unwrap();
    }

    pub fn contains(&self, token: &str) -> bool {
        self.inner.lock().unwrap().contains_key(token)
    }
}

impl Clone for Handler {
    fn clone(&self) -> Self {
        let inner = self.inner.clone();
        Self {
            inner,
        }
    }
}