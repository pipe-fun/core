use chrono::NaiveTime;
use std::collections::VecDeque;
use crate::pipe_task::PipeTask;

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalTask {
    id: i32,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

impl Clone for OriginalTask {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            command: self.command.clone(),
            execute_time: self.execute_time,
            device_token: self.device_token.clone(),
            active: self.active
        }
    }
}

impl OriginalTask {
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn execute_time(&self) -> NaiveTime {
        self.execute_time
    }

    pub fn get_all_task_by_token(token: &str) -> VecDeque<PipeTask> {
        let vec = reqwest::blocking::get("http://localhost:1122/api/task/read").unwrap();
        let vec = vec.json::<Vec<OriginalTask>>().unwrap();

        vec.iter()
            .filter(|v| v.device_token.eq(&token.to_string()))
            .map(|v| v.clone().to_pipe_task())
            .collect()
    }

    pub fn to_pipe_task(self) -> PipeTask {
        PipeTask::from(self)
    }
}