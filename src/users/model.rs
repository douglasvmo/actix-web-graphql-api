use crate::errors::ServiceError;
use crate::errors::ServiceResult;
use crate::schema::auths;
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct UserAuth {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub payload: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub cpf_cnpj: String,
    #[graphql(skip)]
    pub password: String,
    pub role_id: Option<i32>,
    pub active: bool,
    #[graphql(skip)]
    pub verification_payload: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "auths"]
pub struct InsertableLogin {
    pub user_id: uuid::Uuid,
    pub payload: String,
}

#[derive(Debug, Insertable, GraphQLInputObject)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub cpf_cnpj: String,
    pub password: String,
}

impl UserAuth {
    pub fn new_login(user_id: uuid::Uuid, conn: &PgConnection) -> ServiceResult<UserAuth> {
        let n_login = InsertableLogin {
            user_id,
            payload: String::from("login"),
        };

        let login: UserAuth = diesel::insert_into(auths::table)
            .values(&n_login)
            .get_result(conn)?;
        Ok(login)
    }
}

fn hash_encode(password: String, salt: &str) -> String {
    use sha2::Digest;
    let mut sha512 = sha2::Sha512::new();
    sha512.update(password + salt);
    hex::encode(sha512.finalize())
}

impl User {
    pub fn new(mut user_data: NewUser, conn: &PgConnection) -> ServiceResult<User> {
        use self::users::dsl::*;
        user_data.password = hash_encode(user_data.password, &user_data.email);

        Ok(diesel::insert_into(users)
            .values(&user_data)
            .get_result(conn)?)
    }

    pub fn get_users(coon: &PgConnection) -> ServiceResult<Vec<User>> {
        use self::users::dsl::*;
        Ok(users.load::<User>(coon)?)
    }

    pub fn get_by_login(login: String, conn: &PgConnection) -> ServiceResult<User> {
        use self::users::dsl::*;
        let user = users
            .filter(cpf_cnpj.eq(login.clone()))
            .or_filter(email.eq(login))
            .first::<User>(conn)
            .map_err(|_| ServiceError::Unauthorized)?;
        Ok(user)
    }

    pub fn check_password(&self, passwd: String) -> ServiceResult<&User> {
        let hash = hash_encode(passwd, &self.email);

        if self.password == hash {
            Ok(self)
        } else {
            Result::Err(ServiceError::Unauthorized)
        }
    }

    pub fn get_by_id(id: &str, conn: &PgConnection) -> ServiceResult<User> {
        let user_uuid = uuid::Uuid::parse_str(id).map_err(|e| ServiceError::BadRequest)?;
        Ok(users::dsl::users.find(user_uuid).first(conn)?)
    }
}
