use rocket_contrib::json::Json;
use crate::user::*;
use crate::user_repository::*;


pub struct UserService {
    repository: UserRepository
}

pub fn build_user_service(repository: UserRepository) -> UserService{
    UserService{
        repository: repository,
    }
}

impl UserService{
    pub fn add_user(&self,user: Json<User>){
        self.repository.add_user(user.0);
    }
}