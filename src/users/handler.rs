use super::model::{InsertableUser, User};
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use actix_web::web;

impl User {
    pub fn get_users(pool: web::Data<Pool>) -> ServiceResult<Vec<User>> {
        let coon = db_connection(&pool)?;
        Ok(users.load::<User>(&coon)?)
    }

    pub fn register(user: InsertableUser, pool: web::Data<Pool>) -> ServiceResult<User> {
        let conn = db_connection(&pool)?;
        let inserted_user: User = diesel::insert_into(users).values(&user).get_result(&conn)?;
    
        Ok(inserted_user)
    }    
}
