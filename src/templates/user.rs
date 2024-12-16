use crate::database::models::bike::BikeDetail;
use crate::database::models::user::User;
use askama::Template;

const WEAK_PASSWORD_MESSAGE: &str = "Weak password! Password must contain at least one from each: {lower case character, upper case character, number, special character} and must be at least 6 characters long";

#[derive(Template, Default)]
#[template(path = "user/login.html")]
pub struct LoginTemplate {
    pub message: String,
    pub return_url: String,
}

#[derive(Template, Default)]
#[template(path = "user/manage/password/content.html")]
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

#[derive(Template)]
#[template(path = "user/manage/profile/page.html")]
pub struct UserManageProfilePageTemplate {
    pub user: User,
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "user/manage/profile/content.html")]
pub struct UserManageProfileContentTemplate {
    pub user: User,
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}


#[derive(Template)]
#[template(path = "user/profile_user_form.html")]
pub struct UserManageProfileUserFormTemplate {
    pub user: User,
    pub message: String,
    pub success: bool,
    pub logged_in: bool,
}

