use crate::execute_time::ExecuteTime;
use crate::original_task::OriginalTask;

#[derive(Debug)]
pub struct PipeTask {
    ot: OriginalTask,
    id: i32,
    name: String,
    active: bool,
    command: String,
    execute_time: ExecuteTime,
}

impl PipeTask {
    pub fn from(ot: OriginalTask) -> Self {
        PipeTask {
            ot: ot.clone(),
            id: ot.id(),
            name: ot.name(),
            active: ot.active(),
            command: ot.command(),
            execute_time: ExecuteTime::from(ot.execute_time()),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn execute_time(&self) -> ExecuteTime {
        self.execute_time
    }

    pub fn original_task(&self) -> OriginalTask {
        self.ot.clone()
    }
}