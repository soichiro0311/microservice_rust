use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Queryable)]
pub struct User {
    pub userid: String,
    pub password: String,
}
