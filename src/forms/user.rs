use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UserCreateForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub name: String,
    pub surname: String,
}

#[derive(Debug, MultipartForm)]
pub struct ProfilePictureUploadForm {
    #[multipart(rename = "picture")]
    pub picture: TempFile,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UserUpdateForm {
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub bio: String,
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
