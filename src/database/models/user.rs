use serde::Deserialize;
use crate::database::common::EntityById;
use crate::database::models::Id;


#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Id,
    // --------------
    pub email: String,
    pub name: String,
    pub surname: String,
    pub password_hash: String,
    pub password_salt: String,
    pub admin: bool
}

impl EntityById for User {
    fn id(&self) -> Id {
        self.id
    }

    fn is_deleted(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

impl UserLogin {
    #[must_use]
    #[inline]
    pub fn new(email: &str, password_hash: &str) -> Self {
        Self {
            email: email.to_owned(),
            password: password_hash.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserUpdatePassword {
    pub id: Id,
    pub old_password: String,
    pub new_password: String,
}

impl UserUpdatePassword {
    pub fn new(id: &Id, old_password: &str, new_password: &str) -> Self {
        Self {
            id: *id,
            old_password: old_password.to_owned(),
            new_password: new_password.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserUpdate {
    pub id: Id,
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub password: Option<String>,
    pub admin: Option<bool>,
}

impl UserUpdate {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Id,
        email: Option<&str>,
        name: Option<&str>,
        surname: Option<&str>,
        password: Option<&str>,
        admin: Option<bool>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            email: email.and_then(change_to_owned),
            name: name.and_then(change_to_owned),
            surname: surname.and_then(change_to_owned),
            password: password.and_then(change_to_owned),
            admin,
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.email.is_none()
            && self.name.is_none()
            && self.surname.is_none()
            && self.password.is_none()
    }
}
