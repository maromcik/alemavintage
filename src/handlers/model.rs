use crate::database::common::{DbCreate, DbReadMany, DbReadOne};
use crate::database::models::model::{ModelCreate, ModelSearch};
use crate::database::repositories::model::repository::ModelRepository;
use crate::error::AppError;
use crate::forms::model::ModelCreateForm;
use crate::handlers::helpers::get_template_name;
use crate::templates::model::{ModelCreateTemplate, ModelDetailTemplate, ModelTemplate};
use crate::{authorized, AppState};
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable};
use crate::database::models::brand::BrandSearch;
use crate::database::models::{GetById, Id};
use crate::database::models::bike::BikeSearch;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::brand::repository::BrandRepository;

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
    let body = template.render(ModelCreateTemplate { brands, logged_in: true })?;

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
        .create(&ModelCreate::new(&form.brand_id, &form.name, &form.description))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/model"))
        .finish())
}

#[get("")]
pub async fn get_models(
    request: HttpRequest,
    model_repo: web::Data<ModelRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let template_name = get_template_name(&request, "model");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ModelTemplate {
        models,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("{id}/detail")]
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
        .read_many(&BikeSearch::search_by_model_id(model_id, DbQueryParams::order(
            DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc),
            Some(DbTable::Bike),
        )))
        .await?;

    let template_name = get_template_name(&request, "model/detail");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ModelDetailTemplate {
        model,
        bikes,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
