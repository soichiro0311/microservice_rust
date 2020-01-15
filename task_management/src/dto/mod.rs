use super::schema::*;
use crate::domain::*;
use diesel::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDto {
    pub name: String,
    pub time_limit: String,
}

impl TaskDto {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_time_limit(&self) -> &String {
        &self.time_limit
    }
}

#[derive(Debug, Queryable, Insertable)]
pub struct TaskModel {
    pub task_id: String,
    pub name: String,
    pub time_limit: String,
}

impl TaskModel {
    pub fn new(task: Task) -> TaskModel {
        let task_id = format!("{}{}", &task.name, &task.time_limit.to_string());
        return TaskModel {
            task_id: task_id,
            name: task.name,
            time_limit: task.time_limit.to_string(),
        };
    }
}
