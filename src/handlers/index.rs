use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable};
use crate::database::models::bike::{BikeDisplay, BikeSearch};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_template_name;
use crate::templates::index::{AboutTemplate, IndexTemplate};
use crate::AppState;
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};
use crate::database::common::DbReadMany;

#[get("/")]
pub async fn index(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,

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
    
    let template_name = get_template_name(&request, "index");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(IndexTemplate {
        logged_in: identity.is_some(),
        bikes: bikes.into_iter().map(|bike| BikeDisplay::from(bike).description_to_markdown()).collect(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/about")]
pub async fn about(
    request: HttpRequest,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let template_name = get_template_name(&request, "about");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(AboutTemplate {
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
