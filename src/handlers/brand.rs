use crate::database::common::query_parameters::DbQueryParams;
use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::BikeSearch;
use crate::database::models::brand::{BrandCreate, BrandDisplay, BrandSearch, BrandUpdate};
use crate::database::models::model::ModelSearch;
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::error::AppError;
use crate::forms::brand::{BrandCreateForm, BrandEditForm};
use crate::handlers::helpers::{get_template_name, get_user_from_identity, hard_delete_bike};
use crate::templates::brand::{
    BrandCreateTemplate, BrandDetailTemplate, BrandEditTemplate, BrandTemplate,
};
use crate::{authorized, AppState};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use crate::database::repositories::image::repository::ImageRepository;
use crate::database::repositories::user::repository::UserRepository;

#[get("/create")]
pub async fn create_brand_page(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

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
    user_repo: web::Data<UserRepository>,
    form: web::Form<BrandCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let _ = brand_repo
        .create(&BrandCreate::new(&form.name, &form.description))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/brand"))
        .finish())
}

#[get("/{id}/edit")]
pub async fn edit_brand_page(
    request: HttpRequest,
    brand_repo: web::Data<BrandRepository>,
    user_repo: web::Data<UserRepository>,
    identity: Option<Identity>,
    path: web::Path<(Id,)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;
    let brand = brand_repo
        .read_one(&GetById::new(path.into_inner().0))
        .await?;
    let template_name = get_template_name(&request, "brand/edit");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BrandEditTemplate {
        brand: &brand,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/edit")]
pub async fn edit_brand(
    request: HttpRequest,
    identity: Option<Identity>,
    brand_repo: web::Data<BrandRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<BrandEditForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let _ = brand_repo
        .update(&BrandUpdate::new(
            &form.id,
            Some(&form.name),
            Some(&form.description),
        ))
        .await?;

    let url = format!("/brand/{}", form.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, url))
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
        brands: &brands,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("{id}")]
pub async fn get_brand(
    request: HttpRequest,
    model_repo: web::Data<ModelRepository>,
    brand_repo: web::Data<BrandRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let brand_id = path.into_inner().0;
    let brand = brand_repo.read_one(&GetById::new(brand_id)).await?;
    let models = model_repo
        .read_many(&ModelSearch::new(
            Some(&brand_id),
            None,
            DbQueryParams::default(),
        ))
        .await?;

    let template_name = get_template_name(&request, "brand/detail");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BrandDetailTemplate {
        brand: &BrandDisplay::from(brand),
        models: &models,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[delete("/{id}/delete")]
pub async fn remove_brand(
    request: HttpRequest,
    identity: Option<Identity>,
    brand_repo: web::Data<BrandRepository>,
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    image_repo: web::Data<ImageRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;
    
    let brand_id = path.into_inner().0;

    let bikes = bike_repo
        .read_many(&BikeSearch::search_by_brand_id(
            brand_id,
            DbQueryParams::default(),
        ))
        .await?;

    hard_delete_bike(&bike_repo, &image_repo, bikes.iter().map(|b| b.id).collect()).await?;

    let _ = brand_repo
        .delete(&GetById::new_with_deleted(brand_id))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/brand"))
        .finish())
}
