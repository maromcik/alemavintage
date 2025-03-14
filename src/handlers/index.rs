use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable};
use crate::database::common::DbReadMany;
use crate::database::models::bike::{BikeDisplay, BikeSearch};
use crate::database::models::image::{ImageTypeId, OtherImageSearch, OtherImageTypeEnum};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::image::repository::ImageRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_template_name;
use crate::templates::index::{AboutTemplate, IndexTemplate};
use crate::AppState;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/")]
pub async fn index(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    image_repo: web::Data<ImageRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let bikes = bike_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::new(
            Some(DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc)),
            Some(4),
            None,
            identity.is_none().then_some(DbTable::Bike),
        )))
        .await?;

    let images = image_repo.read_many(&OtherImageSearch::new(Some(OtherImageTypeEnum::Homepage.id()))).await?;
    
    let template_name = get_template_name(&request, "index");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(IndexTemplate {
        logged_in: identity.is_some(),
        bikes: &bikes.into_iter().map(|bike| BikeDisplay::from(bike).description_to_markdown()).collect(),
        images: &images
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/about")]
pub async fn about(
    request: HttpRequest,
    identity: Option<Identity>,
    image_repo: web::Data<ImageRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let images = image_repo.read_many(&OtherImageSearch::new(Some(OtherImageTypeEnum::About.id()))).await?;
    
    let template_name = get_template_name(&request, "about");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;

    let body = template.render(AboutTemplate {
        logged_in: identity.is_some(),
        images: &images
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
