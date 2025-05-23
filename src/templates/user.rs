use crate::database::models::user::User;
use serde::Serialize;

const WEAK_PASSWORD_MESSAGE: &str = "Weak Password! Must contain at least one char from: {lower, upper, number, special} and be at least 6 characters long.";

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
pub struct UserManageProfileTemplate<'a> {
    pub user: &'a User,
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct UserManageProfileUserFormTemplate<'a> {
    pub user: &'a User,
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ContactAdminTemplate {
    pub message: String,
}
