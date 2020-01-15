#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod controller;
mod domain;
mod dto;
mod repository;
mod schema;
mod service;

use controller::*;
use dto::*;
use repository::*;
use service::*;

#[post("/task/add", data = "<task>")]
fn task_add(task: Json<TaskDto>) {
    TaskController::new(TaskService::new(TaskRepository::new())).task_add(task);
}

fn main() {
    rocket::ignite().mount("/", routes![task_add]).launch();
}
