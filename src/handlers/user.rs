use crate::authorized;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::templates::user::{
    LoginTemplate, UserManagePasswordTemplate, UserManageProfileContentTemplate,
    UserManageProfilePageTemplate, UserManageProfileUserFormTemplate,
};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::Redirect;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use askama::Template;

use crate::database::common::error::{BackendError, BackendErrorKind};
use crate::database::common::{DbReadOne, DbUpdate};
use crate::database::models::user::{UserLogin, UserUpdate, UserUpdatePassword};
use crate::database::models::GetById;
use crate::forms::user::{
    UserLoginForm, UserLoginReturnURL, UserUpdateForm, UserUpdatePasswordForm,
};

use crate::handlers::utilities::{is_htmx, parse_user_id, validate_password};

#[get("/login")]
pub async fn login(
    identity: Option<Identity>,
    query: web::Query<UserLoginReturnURL>,
) -> Result<HttpResponse, AppError> {
    let return_url = query.ret.clone().unwrap_or("/".to_string());
    if identity.is_some() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, return_url))
            .finish());
    }
    let template = LoginTemplate {
        message: "".to_string(),
        return_url,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/login")]
pub async fn login_user(
    request: HttpRequest,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserLoginForm>,
) -> Result<impl Responder, AppError> {
    match user_repo
        .read_one(&UserLogin::new(&form.email, &form.password))
        .await
    {
        Ok(user) => {
            Identity::login(&request.extensions(), user.id.to_string())?;
            Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, form.return_url.clone()))
                .finish())
        }
        Err(db_error) => {
            let Some(backend_error) = db_error.get_backend_error() else {
                return Err(AppError::from(db_error));
            };

            if backend_error.is_login_error() {
                let template = LoginTemplate {
                    message: backend_error.to_string(),
                    return_url: form.return_url.clone(),
                };
                let body = template.render()?;
                return Ok(HttpResponse::Ok().content_type("text/html").body(body));
            }

            Err(AppError::from(db_error))
        }
    }
}

#[get("/logout")]
pub async fn logout_user(identity: Option<Identity>) -> Result<impl Responder, AppError> {
    if let Some(u) = identity {
        u.logout();
    }
    Ok(Redirect::to("/").using_status_code(StatusCode::FOUND))
}

#[get("/manage")]
pub async fn user_manage_form_page(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user = user_repo.read_one(&GetById::new(parse_user_id(u)?)).await?;

    let body = match is_htmx(request) {
        true => UserManageProfilePageTemplate {
            user,
            message: "".to_string(),
            success: true,
            logged_in: true,
        }
        .render()?,
        false => UserManageProfileContentTemplate {
            user,
            message: "".to_string(),
            success: true,
            logged_in: true,
        }
        .render()?,
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/password")]
pub async fn user_manage_password_form(
    request: HttpRequest,
    identity: Option<Identity>,
) -> Result<impl Responder, AppError> {
    authorized!(identity, request.path());
    let template = UserManagePasswordTemplate {
        message: "".to_string(),
        success: true,
        logged_in: true,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/profile")]
pub async fn user_manage_profile_form(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user = user_repo.read_one(&GetById::new(parse_user_id(u)?)).await?;
    let template = UserManageProfileUserFormTemplate {
        user,
        message: "".to_string(),
        success: true,
        logged_in: true,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/manage")]
pub async fn user_manage(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdateForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user_update = UserUpdate::new(
        &parse_user_id(u)?,
        Some(&form.email),
        Some(&form.name),
        Some(&form.surname),
        None,
        None,
    );
    let user = user_repo.update(&user_update).await?;

    let Some(user_valid) = user.into_iter().next() else {
        return Err(AppError::from(BackendError::new(
            BackendErrorKind::UserUpdateParametersEmpty,
        )));
    };
    let template = UserManageProfileUserFormTemplate {
        user: user_valid,
        message: "Profile update successful".to_string(),
        success: true,
        logged_in: true,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/manage/password")]
pub async fn user_manage_password(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdatePasswordForm>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());

    if form.new_password != form.confirm_password {
        let template = UserManagePasswordTemplate {
            message: "Passwords do not match".to_string(),
            success: false,
            logged_in: true,
        };
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    if !validate_password(&form.new_password) {
        let template = UserManagePasswordTemplate::weak_password();
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    let update_status = user_repo
        .update_password(&UserUpdatePassword::new(
            &parse_user_id(u)?,
            &form.old_password,
            &form.new_password,
        ))
        .await;

    if update_status.is_err() {
        let template = UserManagePasswordTemplate {
            message: "Old password incorrect".to_string(),
            success: false,
            logged_in: true,
        };
        let body = template.render()?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    let template = UserManagePasswordTemplate {
        message: "Password update successful".to_string(),
        success: true,
        logged_in: true,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
