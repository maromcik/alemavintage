use crate::database::common::{DbCreate, DbReadMany};
use crate::database::models::brand::{BrandCreate, BrandSearch};
use crate::database::repositories::brand::repository::BrandRepository;
use crate::error::AppError;
use crate::forms::brand::BrandCreateForm;
use crate::handlers::helpers::get_template_name;
use crate::templates::brand::{BrandCreateTemplate, BrandTemplate};
use crate::{authorized, AppState};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpRequest, HttpResponse};

#[get("/create")]
pub async fn create_brand_page(
    request: HttpRequest,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let template_name = get_template_name(&request, "brand/create");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BrandCreateTemplate { logged_in: true })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_brand(
    request: HttpRequest,
    identity: Option<Identity>,
    brand_repo: web::Data<BrandRepository>,
    form: web::Form<BrandCreateForm>,
) -> Result<HttpResponse, AppError> {
    let _ = authorized!(identity, request.path());

    let _ = brand_repo
        .create(&BrandCreate::new(&form.name, &form.description))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/brand"))
        .finish())
}

#[get("")]
pub async fn get_brands(
    request: HttpRequest,
    brand_repo: web::Data<BrandRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let brands = brand_repo.read_many(&BrandSearch::default()).await?;

    let template_name = get_template_name(&request, "brand");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BrandTemplate {
        brands,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
