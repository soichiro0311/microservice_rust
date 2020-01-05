use serde::{Deserialize, Serialize};
use super::schema::*;

#[derive(Debug, Serialize, Deserialize,Queryable,Insertable)]
pub struct UserInfo {
    pub user_id: String,
    pub password: String,
    pub logined_flg: bool,
}

#[derive(Debug, Serialize, Deserialize,Queryable)]
pub struct UserDto {
    pub request_user_id: String,
    pub userid: String,
    pub password: String,
}

impl UserDto {
    pub fn get_request_user_id(&self) -> &String{
        &self.request_user_id
    }
}

pub fn build_user_info(user: &UserDto) -> UserInfo{
    UserInfo{
        user_id: user.userid.to_string(),
        password: user.password.to_string(),
        logined_flg: false,
    }
}