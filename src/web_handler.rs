use web2core::protoc::{Operation, OpResult};
use async_std::net::{TcpListener, ToSocketAddrs, TcpStream};
use futures::{StreamExt, AsyncReadExt, AsyncWriteExt};
use crate::task_handler::TaskHandler;

pub struct WebHandler {
    task_handler: TaskHandler,
}

impl WebHandler {
    pub fn new(t: TaskHandler) -> Self {
        Self {
            task_handler: t
        }
    }

    pub async fn start<A: ToSocketAddrs>(&mut self, key: &str, addrs: A) -> std::io::Result<()> {
        let web_listener = TcpListener::bind(addrs).await?;
        let key = key.to_string();
        let mut handler = self.task_handler.clone();

        async_std::task::spawn(async move {
            let mut buf = [0; 1024];
            while let Some(stream) = web_listener.incoming().next().await {
                let mut socket = stream.unwrap();
                let len = socket.read(&mut buf).await.unwrap();
                match core::str::from_utf8(&buf[..len]) {
                    Ok(t) => if !t.eq(&key) { continue; },
                    Err(_) => continue,
                };

                loop {
                    let len = match socket.read(&mut buf).await {
                        Ok(len) => len,
                        Err(_) => break
                    };

                    let value = match core::str::from_utf8(&buf[..len]) {
                        Ok(t) => t,
                        Err(_) => continue,
                    };

                    let op = match serde_json::from_str::<Operation>(value) {
                        Ok(o) => o,
                        Err(_) => continue,
                    };

                    println!("{:?}", op);
                    WebHandler::deal(&mut socket.clone(), &mut handler, op).await;
                }
            }
        });

        Ok(())
    }

    async fn deal(socket: &mut TcpStream, handler: &mut TaskHandler, op: Operation) {
        match op {
            Operation::Execute(info) => {
                match handler.execute(info).await {
                    OpResult::Ok => {
                        let buf = serde_json::to_string(&OpResult::Ok)
                            .unwrap();
                        socket.write(buf.as_bytes()).await.unwrap();
                    }
                    OpResult::DeviceOffline => {
                        let buf = serde_json::to_string(&OpResult::DeviceOffline)
                            .unwrap();
                        socket.write(buf.as_bytes()).await.unwrap();
                    }
                    OpResult::CoreOffline => (),
                }
            }
            Operation::Reload(token) => {
                match handler.reload(&token).await {
                    OpResult::Ok => {
                        let buf = serde_json::to_string(&OpResult::Ok)
                            .unwrap();
                        socket.write(buf.as_bytes()).await.unwrap();
                    }
                    OpResult::DeviceOffline => {
                        let buf = serde_json::to_string(&OpResult::DeviceOffline)
                            .unwrap();
                        socket.write(buf.as_bytes()).await.unwrap();
                    }
                    OpResult::CoreOffline => ()
                }
            }
        }
    }
}