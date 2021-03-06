use std::collections::VecDeque;
use std::future::Future;
use async_std::net::TcpStream;
use futures::AsyncWriteExt;
use crate::device::Device;
use crate::pipe_task::PipeTask;
use crate::original_task::OriginalTask;

#[allow(dead_code)]
pub struct PipeTasks {
    owner: String,
    device_token: String,
    tasks: VecDeque<PipeTask>,
    socket: TcpStream,
}

impl PipeTasks {
    pub fn new(token: &str, socket: TcpStream) -> Self {
        let tasks = OriginalTask::get_all_task_by_token(token);
        let owner = Device::get_owner_by_token(token);

        PipeTasks {
            owner,
            device_token: token.into(),
            tasks,
            socket
        }
    }

    pub fn is_invalid(&self) -> bool {
        self.owner.is_empty()
    }

    pub async fn get_all_future(&mut self) -> Vec<impl Future<Output = ()>> {
        let mut tasks = Vec::new();
        let mut new_deque = VecDeque::new();

        while let Some(t) = self.pop() {
            if !t.active() { new_deque.push_back(t); continue; }

            let execute_time = t.execute_time();
            let delay = execute_time.duration();
            let name = t.name();
            let command = t.command();
            let ot = t.original_task();

            let mut socket = self.socket.clone();

            let f = async move {
                println!("task_name: {}, command: {}, execute_time: {}"
                         , name, command, execute_time.time());
                async_std::task::sleep(delay).await;

                match socket.write(command.as_bytes()).await {
                    Ok(_) => {
                        OriginalTask::update_success(ot);
                        println!("task {} has been executed", name);
                    }
                    Err(e) => {
                        OriginalTask::update_failed(ot);
                        println!("execute error: {:?}", e.to_string());
                    }
                }

            };

            new_deque.push_back(t);
            tasks.push(f);
        }

        self.tasks = new_deque;
        tasks
    }

    pub fn pop(&mut self) -> Option<PipeTask> {
        self.tasks.pop_front()
    }
}