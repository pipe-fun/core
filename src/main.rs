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
use dotenv::dotenv;
use std::env;
use crate::task_handler::TaskHandler;
use crate::web_handler::WebHandler;

async fn test() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("WEB_TO_CORE_HOST").expect("WEB_TO_CORE_HOST is not set in .env file");
    let port = env::var("WEB_TO_CORE_PORT").expect("WEB_TO_CORE_PORT is not set in .env file");
    let key = env::var("WEB_TO_CORE_KEY").expect("WEB_TO_CORE_KEY is not set in .env file");

    let core_host = env::var("CORE_HOST").expect("CORE_HOST is not set in .env file");
    let core_port = env::var("CORE_PORT").expect("CORE_PORT is not set in .env file");

    let mut task_handler = TaskHandler::new();
    let mut web_handler = WebHandler::new(task_handler.clone());
    web_handler.start(&key, format!("{}:{}", host, port)).await?;
    task_handler.start(format!("{}:{}", core_host, core_port)).await?;
    Ok(())
}

fn main() {
    block_on(test()).unwrap();
}