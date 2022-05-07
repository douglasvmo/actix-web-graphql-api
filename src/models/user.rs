use crate::schema::users;
use diesel::{Insertable, PgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub cpf_cnpj: String,
    pub password: String,
    pub role_id: Option<i32>,
    pub active: bool,
    pub verification_code: Option<String>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub cpf_cnpj: &'a str,
    pub password: &'a str,
}

impl User {
    pub fn get_all(conn: &PgConnection) -> Vec<User> {
        users::dsl::users
            .load::<User>(conn)
            .expect("load users error")
    }
    pub fn insert(conn: &PgConnection, user: &NewUser) -> Result<User, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(conn)
    }
}
