use chrono::{NaiveTime, NaiveDateTime, Local};
use std::collections::VecDeque;
use crate::pipe_task::PipeTask;
use crate::request;

#[derive(Serialize, Deserialize, Debug)]
pub struct OriginalTask {
    id: i32,
    name: String,
    succeed_count: i32,
    failed_count: i32,
    last_executed: NaiveDateTime,
    owner: String,
    command: String,
    execute_time: NaiveTime,
    device_token: String,
    active: bool,
}

impl Clone for OriginalTask {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            succeed_count: self.succeed_count,
            failed_count: self.failed_count,
            last_executed: self.last_executed,
            owner: self.owner.clone(),
            command: self.command.clone(),
            execute_time: self.execute_time,
            device_token: self.device_token.clone(),
            active: self.active,
        }
    }
}

impl OriginalTask {
    pub fn get_all_task_by_token(token: &str) -> VecDeque<PipeTask> {
        let url = "/task/read";
        let vec: Vec<OriginalTask> = request::get(url);

        vec.iter()
            .filter(|v| v.device_token.eq(&token.to_string()))
            .map(|v| v.clone().to_pipe_task())
            .collect()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

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

    pub fn to_pipe_task(self) -> PipeTask {
        PipeTask::from(self)
    }

    pub fn update_success(mut ot: OriginalTask) {
        ot.succeed_count += 1;
        ot.last_executed = Local::now().naive_local();
        let url = format!("/task/update/{}", ot.id);
        request::put(&url, &ot)
    }
}