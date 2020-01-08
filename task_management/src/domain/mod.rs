use chrono::*;

pub struct Task {
    pub name: String,
    pub time_limit: Date<Local>,
}

impl Task {
    pub fn new(_name: &String, time_limit: &Date<Local>) -> Result<Task, String> {
        let task = Task {
            name: _name.to_string(),
            time_limit: *time_limit,
        };
        if task.is_valid(time_limit) {
            Ok(task)
        } else {
            Err("create task error.".to_string())
        }
    }

    fn is_valid(&self, time_limit: &Date<Local>) -> bool {
        let diff = *time_limit - Local::today();
        if diff.num_days() < 0 {
            false
        } else {
            true
        }
    }
}
