#![feature(duration_zero)]

mod execute_time;
mod original_task;
mod pipe_task;
mod pipe_tasks;
mod device;
mod handler;

#[macro_use]
extern crate serde_derive;

use futures::executor::block_on;
use async_std::net::TcpListener;
use futures::{StreamExt, AsyncReadExt};
use std::sync::mpsc;
use web2core::protoc::Operation;
use crate::pipe_tasks::PipeTasks;
use crate::handler::Handler;

async fn test() -> std::io::Result<()> {
    let key = "key";
    let mut handler = Handler::new();
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    let web_listener = TcpListener::bind("127.0.0.1:4321").await?;
    let (tx, rx) = mpsc::channel::<String>();

    let mut handler2web = handler.clone();
    async_std::task::spawn(async move {
        let mut buf = [0; 1024];
        while let Some(stream) = web_listener.incoming().next().await {
            let mut socket = stream.unwrap();
            let len = socket.read(&mut buf).await.unwrap();
            match core::str::from_utf8(&buf[..len]) {
                Ok(t) => if !t.eq(key) { continue; },
                Err(_) => continue,
            };

            loop {
                let len = socket.read(&mut buf).await.unwrap();
                let value = match core::str::from_utf8(&buf[..len]) {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let op = match serde_json::from_str::<Operation>(value) {
                    Ok(o) => o,
                    Err(_) => continue,
                };
                match op {
                    Operation::Execute(info) => handler2web.execute(info).await,
                    _ => ()
                }
            }
        }
    });

    let _handler = handler.clone();
    async_std::task::spawn(async move {
        while let Ok(token) = rx.recv() {
            let mut handler = _handler.clone();

            let f = async move {
                handler.cancel_guard(&token).await;
                println!("task cancel");
            };

            async_std::task::spawn(async move { f.await; });
        }
    });

    let mut buf = [0; 1024];
    while let Some(stream) = listener.incoming().next().await {
        let mut socket = stream?;
        let len = socket.read(&mut buf).await?;
        if len == 0 { continue; }

        let _socket = socket.clone();
        let token = match core::str::from_utf8(&buf[..len]) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if handler.contains(token) { continue; }

        let mut tasks = PipeTasks::new(token, _socket);
        if tasks.is_empty() { continue; }

        let handle = async_std::task::spawn(async move {
            loop {
                let fs = tasks.get_all_future().await;
                futures::future::join_all(fs).await;
            }
        });

        handler.insert(token.into(), socket.clone(), handle);
        tx.send(token.into()).unwrap();
    }

    Ok(())
}

fn main() {
    block_on(test()).unwrap();
}