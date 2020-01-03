use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::user::*;
use crate::schemas::user_infos::dsl::*;
use crate::models::*;

pub struct UserRepository{
    connection: PgConnection
}

pub fn build_user_repository() -> UserRepository{
    let conn = establish_connection();
    UserRepository{
        connection: conn,
    }
}

impl UserRepository {
    pub fn add_user(&self,new: User){
        let new_info: UserInfo = build_user_info(&new);
        diesel::insert_into(user_infos).values(&new_info).execute(&self.connection).expect("Error saving new user.");
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
