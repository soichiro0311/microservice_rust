#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket_contrib::json::Json;

#[macro_use]
extern crate rocket;

mod user;
mod user_controller;
mod user_service;
mod user_repository;

use user::*;
use user_controller::*;
use user_service::*;
use user_repository::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schemas;


#[post("/user/add", data = "<user>")]
fn user_add(user: Json<User>) {
    build_user_controller(build_user_service(build_user_repository())).user_add(user)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![user_add])
        .launch();
}
