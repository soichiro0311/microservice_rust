use crate::domain::*;
use crate::dto::*;
use crate::schema::task_models::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub struct TaskRepository {
    connection: PgConnection,
}

impl TaskRepository {
    pub fn new() -> TaskRepository {
        TaskRepository {
            connection: establish_connection(),
        }
    }

    pub fn task_add(&self, task: Task) {
        let _model = TaskModel::new(task);
        diesel::insert_into(task_models)
            .values(&_model)
            .execute(&self.connection)
            .expect("Error saving new task.");
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
