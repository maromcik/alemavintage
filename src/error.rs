use crate::database::common::error::{BackendError, BackendErrorKind, DbError, DbErrorKind};
use crate::templates::error::GenericError;
use actix_identity;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use askama::Template;
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};
use std::io::Error;
use std::num::ParseIntError;
use image::ImageError;
use rexiv2::Rexiv2Error;
use thiserror::Error;

/// User facing error type
#[derive(Error, Debug, Serialize, Clone)]
pub enum AppErrorKind {
    #[error("internal server error")]
    InternalServerError,
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
    #[error("templating error")]
    TemplatingError,
    #[error("identity error")]
    IdentityError,
    #[error("session error")]
    SessionError,
    #[error("conflict")]
    Conflict,
    #[error("file error")]
    FileError,
    #[error("unauthorized")]
    Unauthorized,
}

impl From<askama::Error> for AppError {
    fn from(_error: askama::Error) -> Self {
        Self::new(AppErrorKind::TemplatingError, "Templating error")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppError {
    pub app_error_kind: AppErrorKind,
    pub message: String,
}

impl AppError {
    #[must_use]
    #[inline]
    pub fn new(error: AppErrorKind, description: &str) -> Self {
        Self {
            app_error_kind: error,
            message: description.to_owned(),
        }
    }
}

impl From<BackendError> for AppError {
    fn from(value: BackendError) -> Self {
        match value.error_kind {
            BackendErrorKind::UserUpdateParametersEmpty
            | BackendErrorKind::UserDeleted => {
                Self::new(AppErrorKind::BadRequest, value.to_string().as_str())
            }

            BackendErrorKind::UserDoesNotExist => {
                Self::new(AppErrorKind::NotFound, value.to_string().as_str())
            }

            BackendErrorKind::UserPasswordDoesNotMatch
            | BackendErrorKind::UnauthorizedOperation
            | BackendErrorKind::UserPasswordVerificationFailed => {
                Self::new(AppErrorKind::Unauthorized, value.to_string().as_str())
            }

            _ => Self::new(
                AppErrorKind::InternalServerError,
                value.to_string().as_str(),
            ),
        }
    }
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        match e.db_error_kind {
            DbErrorKind::BackendError(backend_error) => AppError::from(backend_error),
            DbErrorKind::UniqueConstraintError => Self::new(AppErrorKind::Conflict, &e.to_string()),
            DbErrorKind::DatabaseError | DbErrorKind::MigrationError => {
                Self::new(AppErrorKind::InternalServerError, &e.to_string())
            }
            DbErrorKind::NotNullError | DbErrorKind::ForeignKeyError => {
                Self::new(AppErrorKind::BadRequest, &e.to_string())
            }
        }
    }
}

impl From<actix_identity::error::LoginError> for AppError {
    fn from(value: actix_identity::error::LoginError) -> Self {
        Self::new(AppErrorKind::IdentityError, value.to_string().as_str())
    }
}

impl From<actix_identity::error::GetIdentityError> for AppError {
    fn from(value: actix_identity::error::GetIdentityError) -> Self {
        Self::new(AppErrorKind::IdentityError, value.to_string().as_str())
    }
}

impl From<actix_session::SessionInsertError> for AppError {
    fn from(value: actix_session::SessionInsertError) -> Self {
        Self::new(AppErrorKind::SessionError, value.to_string().as_str())
    }
}

impl From<minijinja::Error> for AppError {
    fn from(value: minijinja::Error) -> Self {
        Self::new(AppErrorKind::TemplatingError, value.to_string().as_str())
    }
}

impl From<Rexiv2Error> for AppError {
    fn from(value: Rexiv2Error) -> Self {
        Self::new(AppErrorKind::FileError, value.to_string().as_str())
    }
}

impl From<image::ImageError> for AppError {
    fn from(value: ImageError) -> Self {
        Self::new(AppErrorKind::FileError, value.to_string().as_str())
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: Error) -> Self {
        Self::new(AppErrorKind::FileError, value.to_string().as_str())
    }
}

impl From<lofty::LoftyError> for AppError {
    fn from(value: lofty::LoftyError) -> Self {
        Self::new(AppErrorKind::FileError, value.to_string().as_str())
    }
}

impl From<actix_session::SessionGetError> for AppError {
    fn from(value: actix_session::SessionGetError) -> Self {
        Self::new(AppErrorKind::SessionError, value.to_string().as_str())
    }
}

impl From<ParseIntError> for AppError {
    fn from(_: ParseIntError) -> Self {
        Self::new(AppErrorKind::IdentityError, "Invalid User ID")
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error code: {}, Error message: {}",
            self.app_error_kind, self.message
        )
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.app_error_kind {
            AppErrorKind::BadRequest => StatusCode::BAD_REQUEST,
            AppErrorKind::NotFound => StatusCode::NOT_FOUND,
            AppErrorKind::Conflict => StatusCode::CONFLICT,
            AppErrorKind::Unauthorized => StatusCode::UNAUTHORIZED,
            AppErrorKind::TemplatingError
            | AppErrorKind::InternalServerError
            | AppErrorKind::IdentityError
            | AppErrorKind::SessionError
            | AppErrorKind::FileError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        render_generic(self)
    }
}

fn render_generic(error: &AppError) -> HttpResponse {
    let template = GenericError {
        code: error.status_code().to_string(),
        description: error.message.clone(),
    };
    let body = template.render().unwrap_or_default();
    HttpResponse::build(error.status_code())
        .insert_header(ContentType::html())
        .body(body)
}
