use super::model::{InsertableUser, User};
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::users;
use diesel::prelude::*;

impl User {
    pub fn get_users(coon: &PgConnection) -> ServiceResult<Vec<User>> {
        Ok(users::dsl::users.load::<User>(coon)?)
    }

    pub fn register(user_data: InsertableUser, conn: &PgConnection) -> ServiceResult<User> {
        let new_user: User = diesel::insert_into(users::table)
            .values(&user_data)
            .get_result(conn)?;

        Ok(new_user)
    }

    pub fn get_by_login(login: String, conn: &PgConnection) -> ServiceResult<User> {
        let user = users::dsl::users
            .filter(users::cpf_cnpj.eq(login.clone()))
            .or_filter(users::email.eq(login))
            .first::<User>(conn)
            .map_err(|_| ServiceError::Unauthorized)?;
        Ok(user)
    }

    pub fn check_password(&self, paswd: String) -> ServiceResult<&User> {
        if self.password == paswd {
            Ok(self)
        } else {
            Result::Err(ServiceError::Unauthorized)
        }
    }

    pub fn get_id(id: &str, conn: &PgConnection) -> ServiceResult<User> {
        let user_uuid = uuid::Uuid::parse_str(id).map_err(|e| ServiceError::BadRequest)?;
        Ok(users::dsl::users.find(user_uuid).first(conn)?)
    }
}
