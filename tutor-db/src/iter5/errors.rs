use std::fmt::{Display, Formatter};
use log::{debug, error};
use serde::Serialize;
use actix_web::{error, Error, http::StatusCode, HttpResponse};
use sqlx::error::Error as SQLxError;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorError::DBError(msg) => {
                error!("Database error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::ActixError(msg) => {
                error!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorError::NotFound(msg) => {
                debug!("Not found error occurred: {:?}", msg);
                msg.into()
            }
            EzyTutorError::InvalidInput(msg) => {
                debug!("Invalid params received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl Display for EzyTutorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorError::DBError(_) | EzyTutorError::ActixError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorError::NotFound(_) => { StatusCode::NOT_FOUND }
            EzyTutorError::InvalidInput(_) => { StatusCode::BAD_REQUEST }
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response()
        })
    }
}

impl From<actix_web::error::Error> for EzyTutorError {
    fn from(value: Error) -> Self {
        EzyTutorError::ActixError(value.to_string())
    }
}

impl From<SQLxError> for EzyTutorError {
    fn from(value: SQLxError) -> Self {
        match value {
            SQLxError::RowNotFound => {
                EzyTutorError::NotFound("Requested resource not found".to_string())
            },
            _  => {
                EzyTutorError::DBError(value.to_string())
            }
        }
    }
}