use std::time::SystemTime;

use crate::schema::users::dsl::*;
use diesel::{PgConnection, RunQueryDsl};
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(GraphQLInputObject)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String,
    active: bool,
}

impl User {
    pub fn all(conn: &PgConnection) -> Vec<User> {
        users.load::<User>(conn).expect("load users error")
    }
}
