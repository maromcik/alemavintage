use crate::database::common::query_parameters::DbQueryParams;
use crate::database::common::DbReadMany;
use crate::database::models::bike::BikeSearch;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::{authorized, AppState};

use crate::handlers::helpers::get_template_name;
use crate::templates::studio::StudioTemplate;
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, web, HttpRequest, HttpResponse};


#[get("")]
pub async fn studio_index(
    request: HttpRequest,
    identity: Option<Identity>,
    book_repo: web::Data<BikeRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let _ = authorized!(identity, request.path());
    let bikes = book_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::deleted()))
        .await?;

    let template_name = get_template_name(&request, "studio");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(StudioTemplate {
        bikes,
        logged_in: true,
    })?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
