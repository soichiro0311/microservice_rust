use crate::dto::*;
use crate::service::*;
use rocket_contrib::json::Json;

pub struct TaskController {
    service: TaskService,
}

impl TaskController {
    pub fn new(service: TaskService) -> TaskController {
        TaskController { service: service }
    }

    pub fn task_add(&self, task: Json<TaskDto>) {
        self.service.task_add(task.into_inner());
    }
}
