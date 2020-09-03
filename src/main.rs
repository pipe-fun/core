#![feature(duration_zero)]

mod execute_time;
mod original_task;
mod pipe_task;
mod pipe_tasks;
mod device;
mod task_handler;
mod web_handler;

#[macro_use]
extern crate serde_derive;

use futures::executor::block_on;
use crate::task_handler::TaskHandler;
use crate::web_handler::WebHandler;

async fn test() -> std::io::Result<()> {
    let mut task_handler = TaskHandler::new();
    let mut web_handler = WebHandler::new(task_handler.clone());
    web_handler.start("key", "127.0.0.1:4321").await?;
    task_handler.start("0.0.0.0:1234").await?;
    Ok(())
}

fn main() {
    block_on(test()).unwrap();
}