use crate::database::common::repository::DbCreate;
use std::os::unix::raw::mode_t;
use actix_web::http::header::LOCATION;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_bike_detail_base;
use crate::templates::bike::{BikeCreateContentTemplate, BikeCreatePageTemplate, BikeDetailContentTemplate, BikeDetailPageTemplate, BikeUploadFormTemplate};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use askama::Template;
use uuid::Uuid;
use crate::{authorized, parse_host};
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::bike::{BikeCreate, BikeImage, BikeImageCreate};
use crate::database::models::brand::BrandSearch;
use crate::database::models::model::ModelSearch;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::forms::bike::{BikeCreateForm, BikeUploadForm};
use crate::handlers::utilities::{get_metadata_from_session, get_user_from_identity, save_file, validate_file, BikeCreateSessionKeys};

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

#[get("/create")]
pub async fn create_bike_page(
    request: HttpRequest,
    identity: Option<Identity>,
    brand_repo: web::Data<BrandRepository>,
    model_repo: web::Data<ModelRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let brands = brand_repo.read_many(&BrandSearch::default()).await?;
    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let template = BikeCreatePageTemplate {
        brands,
        models,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/create-content")]
pub async fn create_bike_content(
    request: HttpRequest,
    identity: Option<Identity>,
    brand_repo: web::Data<BrandRepository>,
    model_repo: web::Data<ModelRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let brands = brand_repo.read_many(&BrandSearch::default()).await?;
    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let template = BikeCreateContentTemplate {
        brands,
        models,
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/upload")]
pub async fn upload_bike_form(
    request: HttpRequest,
    identity: Option<Identity>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let template = BikeUploadFormTemplate {
        message: "".to_string(),
    };
    let body = template.render()?;
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    session: Session,
    user_repo: web::Data<UserRepository>,
    brand_repo: web::Data<BrandRepository>,
    model_repo: web::Data<ModelRepository>,
    form: web::Form<BikeCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);

    let model = model_repo
        .read_one(&GetById::new(form.model_id))
        .await?;

    let brand = brand_repo
        .read_one(&GetById::new(form.brand_id))
        .await?;

    session.insert(session_keys.name.as_str(), &form.name)?;
    session.insert(session_keys.description.as_str(), &form.description)?;
    session.insert(session_keys.brand_id.as_str(), brand.id)?;
    session.insert(session_keys.model_id.as_str(), model.id)?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/bike/upload"))
        .finish())
}

#[post("/upload")]
pub async fn upload_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    session: Session,
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    MultipartForm(mut form): MultipartForm<BikeUploadForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);

    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let bike_create = BikeCreate::new(
        &metadata.name,
        &metadata.brand_id,
        &metadata.model_id,
        &metadata.description,
    );

    let bike = bike_repo.create(&bike_create).await?;
    let paths = form.photos.into_iter().map(|photo|
        {
            let path = validate_file(&photo, Uuid::new_v4(), "image", "bike")?;
            save_file(photo, &path)?;
            Ok(path)
        }).collect::<Result<Vec<String>, AppError>>()?;
    bike_repo.create(&BikeImageCreate::new(bike.id, paths)).await?;

    session.remove(session_keys.name.as_str());
    session.remove(session_keys.description.as_str());
    session.remove(session_keys.brand_id.as_str());
    session.remove(session_keys.model_id.as_str());

    // let handler = format!("/bike/{}/manage-content", bike.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, '/'.to_string()))
        .finish())
}