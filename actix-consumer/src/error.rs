use actix_web::{http::StatusCode, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "bad request")]
    BadRequest,
    #[display(fmt = "not found")]
    NotFound,
    #[display(fmt = "conflict")]
    Conflict,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Conflict => StatusCode::CONFLICT,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(_: reqwest::Error) -> Self {
        ApiError::InternalError
    }
}
