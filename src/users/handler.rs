use super::model::InsertableUser;
use crate::{
    database::{db_connection, Pool},
    models::user::User,
};
use actix_web::web;
use diesel::prelude::*;

pub async fn register(user_data: web::Json<InsertableUser>, pool: web::Data<Pool>) -> User {
    use crate::schema::users::dsl::*;

    let user = user_data.into_inner();
    let conn = db_connection(&pool);
    let inserted_user: User = diesel::insert_into(users)
        .values(&user)
        .get_result(&conn)
        .expect("Insert User");
    inserted_user.into()
}
