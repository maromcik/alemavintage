use actix_identity::Identity;
use actix_session::Session;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use crate::authorized;
use crate::database::models::brand::{BrandCreate, BrandSearch};
use crate::database::models::model::ModelSearch;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::error::AppError;
use crate::handlers::utilities::{get_user_from_identity, is_htmx, BikeCreateSessionKeys};
use crate::templates::bike::BikeCreateContentTemplate;
use actix_web::http::header::LOCATION;
use crate::templates::brand::{BrandContentTemplate, BrandCreateContentTemplate, BrandCreatePageTemplate, BrandPageTemplate};
use askama::Template;
use crate::database::common::{DbCreate, DbReadMany, DbReadOne};
use crate::database::models::GetById;
use crate::database::repositories::user::repository::UserRepository;
use crate::forms::bike::BikeCreateForm;
use crate::forms::brand::BrandCreateForm;

#[get("/create")]
pub async fn create_brand_page(
    request: HttpRequest,
    identity: Option<Identity>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let body = match is_htmx(request) {
        true => BrandCreateContentTemplate {
            logged_in: true,
        }
            .render()?,
        false => BrandCreatePageTemplate {
            logged_in: true,
        }
            .render()?,
    };

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
    
    let brand = brand_repo.create(&BrandCreate::new(&form.name, &form.description)).await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/brand"))
        .finish())
}

#[get("")]
pub async fn get_brands(
    request: HttpRequest,
    brand_repo: web::Data<BrandRepository>,
) -> Result<HttpResponse, AppError>
{
    let brands = brand_repo.read_many(&BrandSearch::new(None)).await?;

    let body = match is_htmx(request) {
        true => BrandContentTemplate {
            brands,
            logged_in: true,
        }
            .render()?,
        false => BrandPageTemplate {
            brands,
            logged_in: true,
        }
            .render()?,
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}