use pbkdf2;
use std::fmt::{Debug, Display, Formatter};

use BackendErrorKind::*;

#[derive(Debug, Clone)]
pub enum BackendErrorKind {
    // User errors
    UserDoesNotExist,
    UserDeleted,
    UserPasswordDoesNotMatch,
    UserUpdateParametersEmpty,
    UserPasswordVerificationFailed,

    // Bike errors
    BikeDoesNotExist,
    BikeDeleted,
    BikeUpdateParametersEmpty,

    BrandDoesNotExist,
    BrandDeleted,
    BrandUpdateParametersEmpty,

    ModelDoesNotExist,
    ModelDeleted,
    ModelUpdateParametersEmpty,

    #[allow(dead_code)]
    UnauthorizedOperation,
}

impl Display for BackendErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let does_not_exist = |name: &str| format!("The specified {name} does not exist!");
        let deleted = |name: &str| format!("The specified {name} has been deleted!");

        match self {
            UserDoesNotExist => f.write_str(does_not_exist("user").as_str()),
            UserDeleted => f.write_str(deleted("user").as_str()),
            UserPasswordDoesNotMatch => {
                write!(
                    f,
                    "The provided email and password combination is incorrect."
                )
            }
            UserPasswordVerificationFailed => {
                write!(f, "Password verification failed.")
            }
            BikeDoesNotExist => f.write_str(does_not_exist("bike").as_str()),
            BikeDeleted => f.write_str(deleted("bike").as_str()),
            BikeUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Bike update query are incorrect",
                        " (no Bike field would be changed)."
                    )
                )
            }
            ModelDoesNotExist => f.write_str(does_not_exist("model").as_str()),
            ModelDeleted => f.write_str(deleted("model").as_str()),
            ModelUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Model update query are incorrect",
                        " (no model field would be changed)."
                    )
                )
            }
            BrandDoesNotExist => f.write_str(does_not_exist("brand").as_str()),
            BrandDeleted => f.write_str(deleted("brand").as_str()),
            BrandUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for Brand update query are incorrect",
                        " (no brand field would be changed)."
                    )
                )
            }
            UserUpdateParametersEmpty => {
                write!(
                    f,
                    concat!(
                        "The provided parameters for User update query are incorrect",
                        " (no User field would be changed)."
                    )
                )
            }
            UnauthorizedOperation => {
                write!(
                    f,
                    concat!("You are not permitted to execute this operation!")
                )
            }
        }
    }
}

/// Error type representing a Business Logic Error in the database layer ->
/// usually a problem with missing records, insufficient rights for operation, and so on.
#[derive(Clone)]
pub struct BackendError {
    pub error_kind: BackendErrorKind,
}

impl BackendError {
    /// Business Logic Error constructor
    #[must_use]
    #[inline]
    pub const fn new(error: BackendErrorKind) -> Self {
        Self { error_kind: error }
    }

    /// Formatted business logic error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Backend Error: {}", self.error_kind)
    }

    pub fn is_login_error(&self) -> bool {
        matches!(
            &self.error_kind,
            UserDoesNotExist
                | UserDeleted
                | UserPasswordDoesNotMatch
                | UserUpdateParametersEmpty
                | UserPasswordVerificationFailed
        )
    }
}

impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub enum DbErrorKind {
    BackendError(BackendError),
    DatabaseError,
    MigrationError,
    UniqueConstraintError,
    NotNullError,
    ForeignKeyError,
}

#[derive(Clone)]
pub struct DbError {
    pub db_error_kind: DbErrorKind,
    description: String,
}

/// Error encapsulating errors from `sqlx` and our own `BusinessLogicError`, unifying errors from
/// the database without the need of `anyhow` library.
impl DbError {
    /// Database Error constructor
    #[must_use]
    #[inline]
    pub fn new(error: DbErrorKind, description: &str) -> Self {
        Self {
            db_error_kind: error,
            description: description.to_owned(),
        }
    }
    /// Formatted database error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Database Error] {}", self.description)
    }

    pub fn get_backend_error(&self) -> Option<BackendError> {
        match &self.db_error_kind {
            DbErrorKind::BackendError(e) => Some(e.clone()),
            _ => None,
        }
    }
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

/// The database error can be assigned to `dyn Error`
impl std::error::Error for DbError {
    fn description(&self) -> &str {
        &self.description
    }
}

/// Conversion from sqlx error, useful when using `?` operator
// impl From<sqlx::Error> for DbError {
//     fn from(value: sqlx::Error) -> Self {
//         Self::new(BusinessLogicError::new(DatabaseError), &format!("sqlx error: {value}"))
//     }
// }

impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Database(err) => match err.kind() {
                sqlx::error::ErrorKind::ForeignKeyViolation => {
                    Self::new(DbErrorKind::ForeignKeyError, &format!("sqlx error: {err}"))
                }
                sqlx::error::ErrorKind::UniqueViolation => Self::new(
                    DbErrorKind::UniqueConstraintError,
                    &format!("sqlx error: {err}"),
                ),
                sqlx::error::ErrorKind::NotNullViolation => {
                    Self::new(DbErrorKind::NotNullError, &format!("sqlx error: {err}"))
                }
                _ => Self::new(DbErrorKind::DatabaseError, &format!("sqlx error: {err}")),
            },
            _ => Self::new(DbErrorKind::DatabaseError, &format!("sqlx error: {value}")),
        }
    }
}

/// Conversion from sqlx migrate error, useful when using `?` operator
impl From<sqlx::migrate::MigrateError> for DbError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::new(
            DbErrorKind::MigrationError,
            &format!("Migration error: {value}"),
        )
    }
}

/// Conversion from business logic error
impl From<BackendError> for DbError {
    fn from(value: BackendError) -> Self {
        Self::new(
            DbErrorKind::BackendError(value.clone()),
            value.to_string().as_str(),
        )
    }
}

impl From<pbkdf2::password_hash::Error> for DbError {
    fn from(value: pbkdf2::password_hash::Error) -> Self {
        Self::new(
            DbErrorKind::BackendError(BackendError::new(UserPasswordVerificationFailed)),
            value.to_string().as_str(),
        )
    }
}

pub struct EntityError {
    pub deleted: BackendErrorKind,
    pub does_not_exist: BackendErrorKind,
}

impl EntityError {
    pub fn new(deleted: BackendErrorKind, does_not_exist: BackendErrorKind) -> Self {
        Self {
            deleted,
            does_not_exist,
        }
    }
}

/// generic database result
pub type DbResult<T> = Result<T, DbError>;

/// Syntax sugar type denoting a singular result from the database
pub type DbResultSingle<T> = DbResult<T>;
/// Syntax sugar type denoting multiple results from the database
pub type DbResultMultiple<T> = DbResult<Vec<T>>;
