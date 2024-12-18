use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct UserCreateForm {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub name: String,
    pub surname: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserUpdateForm {
    pub email: String,
    pub name: String,
    pub surname: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct UserUpdatePasswordForm {
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Deserialize)]
pub struct UserLoginReturnURL {
    pub ret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLoginForm {
    pub email: String,
    pub password: String,
    pub return_url: String,
}
