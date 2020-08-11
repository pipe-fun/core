use crate::execute_time::ExecuteTime;
use crate::original_task::OriginalTask;

#[derive(Debug)]
pub struct PipeTask {
    id: i32,
    active: bool,
    command: String,
    execute_time: ExecuteTime,
}

impl PipeTask {
    pub fn from(ot: OriginalTask) -> Self {
        PipeTask {
            id: ot.id(),
            active: ot.active(),
            command: ot.command(),
            execute_time: ExecuteTime::from(ot.execute_time())
        }
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

    pub fn execute_time(&self) -> ExecuteTime {
        self.execute_time
    }
}