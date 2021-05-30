use crate::schema::users;
use chrono::prelude::{DateTime, Utc};
use diesel::{Insertable, PgConnection, RunQueryDsl};
use juniper::GraphQLInputObject;
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

#[juniper::graphql_object]
impl User {
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn email(&self) -> &str {
        self.email.as_str()
    }
    fn active(&self) -> bool {
        self.active
    }
    fn createAt(&self) -> String {
        let dt: DateTime<Utc> = self.created_at.clone().into();
        dt.format("%+").to_string()
    }
    fn updatedAt(&self) -> String {
        let dt: DateTime<Utc> = self.updated_at.clone().into();
        dt.format("%+").to_string()
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,   
}

#[derive(GraphQLInputObject)]
pub struct CreateUserInput {
    pub name:String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn all(conn: &PgConnection) -> Vec<User> {
        users::dsl::users.load::<User>(conn).expect("load users error")
    }
}
