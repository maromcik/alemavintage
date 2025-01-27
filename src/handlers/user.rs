use crate::database::common::error::{BackendError, BackendErrorKind};
use crate::database::common::{DbReadOne, DbUpdate};
use crate::database::models::user::{UserLogin, UserUpdate, UserUpdatePassword};
use crate::database::models::GetById;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::forms::user::{ContactAdminBikeForm, ContactAdminGeneralForm, UserLoginForm, UserLoginReturnURL, UserUpdateForm, UserUpdatePasswordForm};
use crate::handlers::helpers::{contact_admin_helper, get_template_name, parse_user_id};
use crate::handlers::utilities::validate_password;
use crate::templates::user::{
    LoginTemplate, UserManagePasswordTemplate, UserManageProfileTemplate,
    UserManageProfileUserFormTemplate,
};
use crate::{authorized, AppState};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::Redirect;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};

#[get("/login")]
pub async fn login(
    request: HttpRequest,
    identity: Option<Identity>,
    query: web::Query<UserLoginReturnURL>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let referer = request
        .headers()
        .get(actix_web::http::header::REFERER)
        .map_or("/", |header_value| header_value.to_str().unwrap_or("/"));

    let return_url = query.ret.clone().unwrap_or(referer.to_string());
    if identity.is_some() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, return_url))
            .finish());
    }

    let template_name = "user/login.html";
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(template_name)?;
    let body = template.render(LoginTemplate {
        message: String::new(),
        return_url,
    })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/login")]
pub async fn login_user(
    request: HttpRequest,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserLoginForm>,
    state: web::Data<AppState>,
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
                let template_name = "user/login.html";
                let env = state.jinja.acquire_env()?;
                let template = env.get_template(template_name)?;
                let body = template.render(LoginTemplate {
                    message: backend_error.to_string(),
                    return_url: form.return_url.clone(),
                })?;

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
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user = user_repo
        .read_one(&GetById::new(parse_user_id(&u)?))
        .await?;

    let template_name = get_template_name(&request, "user/manage/profile");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(UserManageProfileTemplate {
        user: &user,
        message: String::new(),
        success: true,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/password")]
pub async fn user_manage_password_form(
    request: HttpRequest,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    authorized!(identity, request.path());

    let template_name = "user/manage/password/content.html";
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(UserManagePasswordTemplate {
        message: String::new(),
        success: true,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/manage/profile")]
pub async fn user_manage_profile_form(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user = user_repo
        .read_one(&GetById::new(parse_user_id(&u)?))
        .await?;

    let template_name = get_template_name(&request, "user/manage/profile");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(UserManageProfileUserFormTemplate {
        user: &user,
        message: String::new(),
        success: true,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/manage")]
pub async fn user_manage(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdateForm>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());
    let user_update = UserUpdate::new(
        &parse_user_id(&u)?,
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

    let template_name = get_template_name(&request, "user/manage/profile");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(UserManageProfileUserFormTemplate {
        user: &user_valid,
        message: "Profile successfully updated".to_string(),
        success: true,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/manage/password")]
pub async fn user_manage_password(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<UserUpdatePasswordForm>,
    state: web::Data<AppState>,
) -> Result<impl Responder, AppError> {
    let u = authorized!(identity, request.path());

    let template_name = "user/manage/password/content.html";
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(template_name)?;

    if form.new_password != form.confirm_password {
        let context = UserManagePasswordTemplate {
            message: "Passwords do not match".to_string(),
            success: false,
            logged_in: true,
        };

        let body = template.render(context)?;

        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    if !validate_password(&form.new_password) {
        let context = UserManagePasswordTemplate::weak_password();
        let body = template.render(context)?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    let update_status = user_repo
        .update_password(&UserUpdatePassword::new(
            &parse_user_id(&u)?,
            &form.old_password,
            &form.new_password,
        ))
        .await;

    if update_status.is_err() {
        let context = UserManagePasswordTemplate {
            message: "Old password incorrect".to_string(),
            success: false,
            logged_in: true,
        };
        let body = template.render(context)?;
        return Ok(HttpResponse::Ok().content_type("text/html").body(body));
    }

    let context = UserManagePasswordTemplate {
        message: "Password successfully updated".to_string(),
        success: true,
        logged_in: true,
    };
    let body = template.render(context)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/contact/bike")]
pub async fn contact_admin_bike(
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
    form: web::Form<ContactAdminBikeForm>,
) -> Result<HttpResponse, AppError> {
    contact_admin_helper(user_repo, bike_repo, identity, state, &form.0).await
}

#[post("/contact/about")]
pub async fn contact_admin_general(
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
    form: web::Form<ContactAdminGeneralForm>,
) -> Result<HttpResponse, AppError> {
    contact_admin_helper(user_repo, bike_repo, identity, state, &form.0).await
}