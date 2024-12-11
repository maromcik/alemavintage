use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_index_base;
use crate::templates::index::{IndexContentTemplate, IndexTemplate};
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn index(
    request: HttpRequest,
    bike_repo: web::Data<BikeRepository>,
) -> Result<HttpResponse, AppError> {
    println!("KURVA");
    let base = get_index_base(bike_repo).await?;
    let body = IndexTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/home-content")]
pub async fn index_content(
    request: HttpRequest,
    bike_repo: web::Data<BikeRepository>,
) -> Result<HttpResponse, AppError> {
    let base = get_index_base(bike_repo).await?;
    let body = IndexContentTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
