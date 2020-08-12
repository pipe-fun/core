use chrono::{NaiveTime, Local};
use std::time::Duration;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct ExecuteTime {
    time: NaiveTime
}

impl ExecuteTime {
    pub fn from(time: NaiveTime) -> Self {
        ExecuteTime {
            time
        }
    }

    pub fn time(&self) -> NaiveTime {
        self.time
    }

    pub fn duration(&self) -> Duration {
        let now_time = Local::now().time();
        match self.time.sub(now_time).to_std() {
            Ok(d) => { d },
            Err(_) => {
                let d = now_time.sub(self.time).to_std().unwrap();
                let d = Duration::from_secs(24 * 60 * 60).sub(d);
                d
            }
        }
    }
}