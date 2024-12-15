use crate::authorized;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_studio;
use crate::templates::studio::{StudioContentTemplate, StudioPageTemplate};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[get("/studio")]
pub async fn studio_index(
    request: HttpRequest,
    identity: Option<Identity>,
    _user_repo: web::Data<UserRepository>,
    book_repo: web::Data<BikeRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = StudioPageTemplate {
        bikes: get_studio(book_repo).await?,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/studio-content")]
pub async fn studio_get_content(
    request: HttpRequest,
    identity: Option<Identity>,
    book_repo: web::Data<BikeRepository>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let template = StudioContentTemplate {
        bikes: get_studio(book_repo).await?,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
