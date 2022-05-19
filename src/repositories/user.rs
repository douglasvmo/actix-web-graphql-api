use crate::database::{get_conn, PoolConnection};
use crate::errors::ServiceResult;
use crate::models::user::{hash_encode, NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use std::sync::Arc;

pub struct UserRepository {
    pool: Arc<PoolConnection>,
}

impl UserRepository {
    pub fn new(pool: Arc<PoolConnection>) -> Self {
        Self { pool }
    }

    pub fn create(&self, mut input: NewUser) -> ServiceResult<User> {
        input.password = hash_encode(input.password, &input.email);
        let conn = get_conn(&self.pool)?;
        Ok(diesel::insert_into(users::dsl::users)
            .values(&input)
            .get_result(&conn)?)
    }

    pub fn all(&self) -> ServiceResult<Vec<User>> {
        use self::users::dsl::*;
        let conn = get_conn(&self.pool)?;
        Ok(users.load::<User>(&conn)?)
    }

    pub fn find(&self, user_id: uuid::Uuid) -> ServiceResult<User> {
        use self::users::dsl::*;
        let conn = get_conn(&self.pool)?;
        Ok(users.find(user_id).first(&conn)?)
    }

    pub fn find_by_login(&self, login: String) -> ServiceResult<User> {
        use self::users::dsl::*;
        let conn = get_conn(&self.pool)?;
        Ok(users
            .filter(email.eq(login.clone()))
            .or_filter(cpf_cnpj.eq(login))
            .first(&conn)?)
    }
}
