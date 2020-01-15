use crate::domain::*;
use crate::dto::*;
use crate::repository::*;

pub struct TaskService {
    repository: TaskRepository,
}
impl TaskService {
    pub fn new(repository: TaskRepository) -> TaskService {
        TaskService {
            repository: repository,
        }
    }

    pub fn task_add(&self, task: TaskDto) {
        let task = Task::new(task.get_name(), task.get_time_limit());
        self.repository.task_add(task.ok().unwrap());
    }
}
