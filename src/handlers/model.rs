use crate::database::common::query_parameters::{
    DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable,
};
use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::BikeSearch;
use crate::database::models::brand::BrandSearch;
use crate::database::models::model::{
    ModelCreate, ModelDetail, ModelDisplay, ModelSearch, ModelUpdate,
};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::error::AppError;
use crate::forms::model::{ModelCreateForm, ModelEditForm};
use crate::handlers::helpers::{get_template_name, hard_delete_bike};
use crate::templates::model::{
    ModelCreateTemplate, ModelDetailTemplate, ModelEditTemplate, ModelTemplate,
};
use crate::{authorized, AppState};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use itertools::Itertools;
use std::collections::HashMap;

#[get("/create")]
pub async fn create_model_page(
    request: HttpRequest,
    identity: Option<Identity>,
    brand_repo: web::Data<BrandRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let brands = brand_repo.read_many(&BrandSearch::new(None)).await?;

    let template_name = get_template_name(&request, "model/create");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ModelCreateTemplate {
        brands,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_model(
    request: HttpRequest,
    identity: Option<Identity>,
    model_repo: web::Data<ModelRepository>,
    form: web::Form<ModelCreateForm>,
) -> Result<HttpResponse, AppError> {
    let _ = authorized!(identity, request.path());

    let _ = model_repo
        .create(&ModelCreate::new(
            &form.brand_id,
            &form.name,
            &form.description,
        ))
        .await?;
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/model"))
        .finish())
}

#[get("/{id}/edit")]
pub async fn edit_model_page(
    request: HttpRequest,
    identity: Option<Identity>,
    model_repo: web::Data<ModelRepository>,
    brand_repo: web::Data<BrandRepository>,
    state: web::Data<AppState>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let model_id = path.into_inner().0;
    let model = model_repo.read_one(&GetById::new(model_id)).await?;
    let brands = brand_repo.read_many(&BrandSearch::new(None)).await?;

    let template_name = get_template_name(&request, "model/edit");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ModelEditTemplate {
        model,
        brands,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/edit")]
pub async fn edit_model(
    request: HttpRequest,
    identity: Option<Identity>,
    model_repo: web::Data<ModelRepository>,
    form: web::Form<ModelEditForm>,
) -> Result<HttpResponse, AppError> {
    let _ = authorized!(identity, request.path());

    let _ = model_repo
        .update(&ModelUpdate::new(
            &form.id,
            Some(&form.brand_id),
            Some(&form.name),
            Some(&form.description),
        ))
        .await?;
    let url = format!("/model/{}", form.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, url))
        .finish())
}

#[get("")]
pub async fn get_models(
    request: HttpRequest,
    model_repo: web::Data<ModelRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let models = model_repo
        .read_many(&ModelSearch::new(
            None,
            None,
            DbQueryParams::order(
                DbOrderColumn::new_column_only(DbColumn::BrandName, DbOrder::Asc),
                None,
            ),
        ))
        .await?;
    
    let models_grouped = models
        .into_iter()
        .chunk_by(|m| m.brand_name.clone())
        .into_iter()
        .map(|group| (group.0, group.1.collect::<Vec<ModelDetail>>()))
        .collect::<HashMap<String, Vec<ModelDetail>>>();
    
    let template_name = get_template_name(&request, "model");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ModelTemplate {
        models: models_grouped,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("{id}")]
pub async fn get_model(
    request: HttpRequest,
    model_repo: web::Data<ModelRepository>,
    bike_repo: web::Data<BikeRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let model_id = path.into_inner().0;
    let model = model_repo.read_one(&GetById::new(model_id)).await?;

    let bikes = bike_repo
        .read_many(&BikeSearch::search_by_model_id(
            model_id,
            DbQueryParams::order(
                DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc),
                identity.is_none().then_some(DbTable::Bike),
            ),
        ))
        .await?;

    let template_name = get_template_name(&request, "model/detail");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ModelDetailTemplate {
        model: ModelDisplay::from(model),
        bikes,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[delete("/{id}/delete")]
pub async fn remove_model(
    request: HttpRequest,
    identity: Option<Identity>,
    model_repo: web::Data<ModelRepository>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let _ = authorized!(identity, request.path());
    let model_id = path.into_inner().0;

    let bikes = bike_repo
        .read_many(&BikeSearch::search_by_model_id(
            model_id,
            DbQueryParams::default(),
        ))
        .await?;

    hard_delete_bike(&bike_repo, bikes.iter().map(|b| b.id).collect()).await?;

    let _ = model_repo
        .delete(&GetById::new_with_deleted(model_id))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/model"))
        .finish())
}
