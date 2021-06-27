use crate::schema::users;
use diesel::{Insertable, PgConnection, RunQueryDsl};
use std::time::SystemTime;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
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
