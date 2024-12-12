use crate::database::models::Id;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_bike_detail_base;
use crate::templates::bike::{BikeDetailContentTemplate, BikeDetailPageTemplate};
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[get("/{id}/detail")]
pub async fn get_bike_detail(
    request: HttpRequest,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let base = get_bike_detail_base(
        bike_repo,
        path.into_inner().0,
    )
        .await?;
    
    let body = BikeDetailPageTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}/detail-content")]
pub async fn get_bike_detail_content(
    request: HttpRequest,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let base = get_bike_detail_base(
        bike_repo,
        path.into_inner().0,
    )
        .await?;

    let body = BikeDetailContentTemplate::from(base).render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}