#![feature(duration_zero)]

mod execute_time;
mod original_task;
mod pipe_task;
mod pipe_tasks;
mod device;

#[macro_use]
extern crate serde_derive;

use futures::executor::block_on;
use std::collections::HashSet;
use async_std::net::{TcpListener, TcpStream};
use futures::{StreamExt, AsyncReadExt};
use async_std::task::JoinHandle;
use std::sync::{mpsc, Mutex};
use async_std::sync::Arc;
use crate::pipe_tasks::PipeTasks;

async fn test() -> std::io::Result<()> {
    let mut buf = [0; 1024];
    let list = Arc::new(Mutex::new(HashSet::new()));
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    let (tx, rx) = mpsc::channel::<(String, TcpStream, JoinHandle<()>)>();

    let _list = list.clone();
    async_std::task::spawn(async move {
        while let Ok((token, mut socket, handle)) = rx.recv() {
            let list = _list.clone();

            let f = async move {
                let mut buf = [];
                socket.read(&mut buf).await.unwrap();
                handle.cancel().await;
                list.lock().unwrap().remove(&token);
                println!("destroy task");
            };

            async_std::task::spawn(async move { f.await; });
        }
    });

    while let Some(stream) = listener.incoming().next().await {
        let mut socket = stream?;
        let len = socket.read(&mut buf).await?;
        if len == 0 { continue; }

        let _socket = socket.clone();
        let token = match core::str::from_utf8(&buf[..len]) {
            Ok(t) => t,
            Err(_) => continue,
        };
        if list.lock().unwrap().contains(token) { continue; }

        let mut tasks = PipeTasks::new(token, _socket);
        if tasks.is_empty() { continue; }
        list.lock().unwrap().insert(token.to_string());

        let handle = async_std::task::spawn(async move {
            loop {
                let fs = tasks.get_all_future().await;
                futures::future::join_all(fs).await;
            }
        });

        tx.send((token.into(), socket, handle))?;
    }

    Ok(())
}

fn main() {
    block_on(test()).unwrap();
}