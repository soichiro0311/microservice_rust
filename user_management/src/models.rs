use serde::{Deserialize, Serialize};
use super::schemas::*;
use crate::user::*;

#[derive(Debug, Serialize, Deserialize,Queryable,Insertable)]
pub struct UserInfo {
    pub user_id: String,
    pub password: String,
    pub logined_flg: bool,
}

pub fn build_user_info(user: &User) -> UserInfo{
    UserInfo{
        user_id: user.userid.to_string(),
        password: user.password.to_string(),
        logined_flg: false,
    }
}