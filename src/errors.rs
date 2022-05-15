use actix_web::{HttpResponse, ResponseError, http};
use diesel::result::Error as DBError;
use juniper::ScalarValue;
use serde::Serialize;
use std::convert::From;
use thiserror::Error;

#[derive(Debug, Error, Serialize, derive_more::Display)]
pub enum ServiceError {
    InternalServerError,
    BadRequest,
    Unauthorized,
    UnableToConnectToDb,
}

impl<S: ScalarValue> juniper::IntoFieldError<S> for ServiceError {
    fn into_field_error(self) -> juniper::FieldError<S> {
        use juniper::graphql_value;
        match self {
            ServiceError::BadRequest => juniper::FieldError::new(
                "Not expected",
                graphql_value!({
                    "type": "BAD_REQUEST"
                }),
            ),
            ServiceError::InternalServerError => juniper::FieldError::new(
                "Internal Error",
                graphql_value!({
                    "type": "INTERNAL_ERROR"
                }),
            ),
            ServiceError::UnableToConnectToDb => juniper::FieldError::new(
                "Unable to connect to DB",
                graphql_value!({
                    "type": "DB_CONNECTION_ERROR"
                }),
            ),
            ServiceError::Unauthorized => juniper::FieldError::new(
                "Unauthorized",
                graphql_value!({
                    "type": "NO_ACCESS"
                }),
            ),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::UnableToConnectToDb => HttpResponse::InternalServerError()
                .json("Unable to connect to DB, Please try later"),
            ServiceError::BadRequest => HttpResponse::BadRequest().json(""),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }

    fn status_code(&self) -> http::StatusCode {
        match self {
            ServiceError::InternalServerError => http::StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::UnableToConnectToDb => http::StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest => http::StatusCode::BAD_REQUEST,
            ServiceError::Unauthorized => http::StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> Self {
        match error {
            DBError::DatabaseError(_kind, info) => ServiceError::BadRequest,
            _ => ServiceError::InternalServerError,
        }
    }
}

pub type ServiceResult<V> = std::result::Result<V, ServiceError>;
