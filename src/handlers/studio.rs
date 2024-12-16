use std::convert::identity;
use crate::authorized;
use crate::database::common::query_parameters::{DbQueryParams, DbTable};
use crate::database::common::DbReadMany;
use crate::database::models::bike::BikeSearch;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::is_htmx;
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
    let _ = authorized!(identity, request.path());
    let bikes = book_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::deleted()))
        .await?;

    let body = match is_htmx(request) {
        true => StudioContentTemplate { bikes, logged_in: true }.render()?,
        false => StudioPageTemplate { bikes, logged_in: true }.render()?,
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
