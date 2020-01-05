use crate::dto::*;
use crate::repository::*;


pub struct UserService {
    repository: UserRepository
}

pub fn build_user_service(repository: UserRepository) -> UserService{
    UserService{
        repository: repository,
    }
}

impl UserService{
    pub fn add_user(&self,user: UserDto){
        if self.isLogined(&user) {
            self.repository.add_user(user);
        }else{
            println!("Not Logined yet. RequestUserId: {}", user.get_request_user_id());
        }
    }

    fn isLogined(&self,user: &UserDto) -> bool{
        let target: UserInfo = self.repository.find_by_id(user.get_request_user_id());
        return target.logined_flg
    }
}