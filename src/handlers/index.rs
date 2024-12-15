use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_index_base;
use crate::handlers::utilities::is_htmx;
use crate::templates::index::{IndexContentTemplate, IndexTemplate};
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn index(
    request: HttpRequest,
    bike_repo: web::Data<BikeRepository>,
) -> Result<HttpResponse, AppError> {
    let base = get_index_base(bike_repo).await?;
    let body = match is_htmx(request) {
        true => IndexContentTemplate::from(base).render()?,
        false => IndexTemplate::from(base).render()?,
    };
    
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
