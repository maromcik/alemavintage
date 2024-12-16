use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable};
use crate::database::common::DbReadMany;
use crate::database::models::bike::BikeSearch;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::utilities::is_htmx;
use crate::templates::index::{IndexBase, IndexContentTemplate, IndexTemplate};
use actix_identity::Identity;
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[get("/")]
pub async fn index(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
) -> Result<HttpResponse, AppError> {
    let bikes = bike_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::order(
            DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc),
            Some(DbTable::Bike),
        )))
        .await?;

    let template = IndexBase {
        logged_in: identity.is_some(),
    };
    
    let body = match is_htmx(request) {
        true => IndexContentTemplate::from(template).render()?,
        false => IndexTemplate::from(template).render()?,
    };
    
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
