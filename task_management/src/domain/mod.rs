use chrono::prelude::*;

pub struct Task {
    pub name: String,
    pub time_limit: Date<Local>,
}

impl Task {
    pub fn new(_name: &String, time_limit: &String) -> Result<Task, String> {
        let date = to_date(time_limit);
        let task = Task {
            name: _name.to_string(),
            time_limit: date,
        };
        if task.is_valid(&task.time_limit) {
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

fn to_date(date_str: &String) -> Date<Local> {
    let year_str = &date_str.chars().take(4).collect::<String>();
    let month_str = &date_str.chars().skip(4).take(2).collect::<String>();
    let day_str = &date_str.chars().skip(6).take(2).collect::<String>();
    return Local.ymd(
        year_str.parse().unwrap(),
        month_str.parse().unwrap(),
        day_str.parse().unwrap(),
    );
}
