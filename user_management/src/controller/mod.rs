use crate::model::*;
use crate::service::*;
use rocket_contrib::json::Json;

pub struct UserController {
    service: UserService,
}

pub fn build_user_controller(service: UserService) -> UserController{
    UserController{
        service: service
    }
}

impl UserController {
    pub fn user_add(&self,user: Json<User>) {
        self.service.add_user(user)
    }
}