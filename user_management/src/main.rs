#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket_contrib::json::Json;

#[macro_use]
extern crate rocket;

mod service;
mod repository;
mod controller;
mod dto;
mod schema;

use controller::*;
use service::*;
use repository::*;
use dto::*;

#[macro_use]
extern crate diesel;
extern crate dotenv;

#[post("/user/add", data = "<user>")]
fn user_add(user: Json<UserDto>) {
    build_user_controller(build_user_service(build_user_repository())).user_add(user)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![user_add])
        .launch();
}
