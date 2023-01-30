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
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict => StatusCode::CONFLICT,
        }
    }
}
