use super::model::{InsertableUser, User, UserLogin};
use crate::errors::{ServiceError, ServiceResult};
use crate::schema::users;

use diesel::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

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

    pub fn Login(login_data: UserLogin, conn: &PgConnection) -> ServiceResult<Option<String>> {
        let user = users::dsl::users
            .filter(users::cpf_cnpj.eq(login_data.login))
            .first::<User>(conn)?;

        if user.password == login_data.password {
            let chaims = crate::jwt::model::Claims::new(user.id, 1);
            let head = Header::new(Algorithm::HS512);
            let key = EncodingKey::from_secret("cat".as_ref());
            let token = encode(&head, &chaims, &key).map_err(|_| ServiceError::Unauthorized)?;
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
}
