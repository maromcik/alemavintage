use crate::database::models::user::User;
use serde::Serialize;

const WEAK_PASSWORD_MESSAGE: &str = "Weak password! Password must contain at least one from each: {lower case character, upper case character, number, special character} and must be at least 6 characters long";

#[derive(Serialize)]
pub struct LoginTemplate {
    pub message: String,
    pub return_url: String,
}

#[derive(Serialize)]
pub struct UserManagePasswordTemplate {
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}

impl UserManagePasswordTemplate {
    pub fn weak_password() -> Self {
        Self {
            success: false,
            message: WEAK_PASSWORD_MESSAGE.to_owned(),
            logged_in: false,
        }
    }
}

#[derive(Serialize)]
pub struct UserManageProfileTemplate {
    pub user: User,
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct UserManageProfileUserFormTemplate {
    pub user: User,
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ContactAdminTemplate {
    pub message: String,
}
