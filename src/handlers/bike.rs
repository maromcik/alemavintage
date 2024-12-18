use crate::database::common::query_parameters::{
    DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable,
};
use crate::database::common::repository::DbCreate;
use crate::database::common::{DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::{
    BikeCreate, BikeCreateSessionKeys, BikeDetail, BikeDetailSessionKeys, BikeDisplay, BikeGetById,
    BikeImageSearch, BikeSearch, BikeUpdate,
};
use crate::database::models::model::ModelSearch;
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::forms::bike::{BikeCreateForm, BikeEditForm, BikeThumbnailEditForm, BikeUploadForm};
use crate::handlers::helpers::{
    bike_hard_delete, get_metadata_from_session, get_template_name, get_user_from_identity,
    upload_bike_helper,
};
use crate::handlers::utilities::{remove_file, save_file, validate_file, ImageDimensions};
use crate::templates::bike::{
    BikeCreateTemplate, BikeDisplayTemplate, BikeEditTemplate, BikeThumbnailUploadTemplate,
    BikeUploadFormTemplate, BikesTemplate,
};
use crate::{authorized, AppState, THUMBNAIL_SIZE};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use uuid::Uuid;

#[get("")]
pub async fn get_bikes(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let bikes = bike_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::order(
            DbOrderColumn::new_column_only(DbColumn::CreatedAt, DbOrder::Desc),
            identity.is_none().then_some(DbTable::Bike),
        )))
        .await?;

    let template_name = get_template_name(&request, "bike");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikesTemplate {
        logged_in: identity.is_some(),
        bikes: bikes.into_iter().map(BikeDisplay::from).collect(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}")]
pub async fn get_bike_detail(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    state: web::Data<AppState>,
    session: Session,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let bike_id = path.into_inner().0;

    let session_keys = BikeDetailSessionKeys::new(bike_id);

    let params = match session.get::<bool>(session_keys.visited.as_str())? {
        None => {
            session.insert(session_keys.visited, true)?;
            BikeGetById::new(bike_id, identity.is_some(), identity.is_none())
        }
        Some(_) => BikeGetById::new(bike_id, identity.is_some(), false),
    };

    let bike: BikeDetail = <BikeRepository as DbReadOne<BikeGetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &params,
    )
    .await?;

    let bike_images: Vec<String> = bike_repo
        .read_many(&BikeImageSearch::new(Some(bike.id)))
        .await?
        .into_iter()
        .map(|image| image.path)
        .collect();

    let template_name = get_template_name(&request, "bike/detail");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeDisplayTemplate {
        bike: BikeDisplay::from(bike),
        bike_images,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/create")]
pub async fn create_bike_page(
    request: HttpRequest,
    identity: Option<Identity>,
    model_repo: web::Data<ModelRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let template_name = get_template_name(&request, "bike/admin/create");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeCreateTemplate {
        models,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/upload")]
pub async fn upload_bike_form(
    request: HttpRequest,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let template_name = get_template_name(&request, "bike/admin/upload");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeUploadFormTemplate {
        message: String::new(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/create")]
pub async fn create_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    session: Session,
    bike_repo: web::Data<BikeRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<BikeCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);

    let bike_create = BikeCreate::new(&form.name, form.model_id, "", &form.description, );

    let bike = bike_repo.create(&bike_create).await?;

    session.insert(&session_keys.bike_id, bike.id)?;

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
    MultipartForm(form): MultipartForm<BikeUploadForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);
    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let bike = match upload_bike_helper(metadata.bike_id, &bike_repo, form).await {
        Ok(bike) => bike,
        Err(err) => {
            bike_hard_delete(&bike_repo, vec![metadata.bike_id]).await?;
            let template_name = get_template_name(&request, "bike/admin/upload");
            let env = state.jinja.acquire_env()?;
            let template = env.get_template(&template_name)?;
            let body = template.render(BikeUploadFormTemplate {
                message: err.to_string(),
            })?;
            return Ok(HttpResponse::Ok().content_type("text/html").body(body));
        }
    };

    session.remove(session_keys.bike_id.as_str());

    let url = format!("/bike/{}", bike.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, url))
        .finish())
}

#[delete("/{id}/delete")]
pub async fn remove_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let bike_id = path.into_inner().0;
    bike_repo
        .delete(&GetById::new_with_deleted(bike_id))
        .await?;

    let path = format!("/bike/{}", bike_id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[delete("/{id}/hard-delete")]
pub async fn hard_remove_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let bike_id = path.into_inner().0;

    bike_hard_delete(&bike_repo, vec![bike_id]).await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/bike"))
        .finish())
}

#[put("/{id}/restore")]
pub async fn restore_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let bike_id = path.into_inner().0;
    bike_repo
        .restore(&GetById::new_with_deleted(bike_id))
        .await?;
    let path = format!("/bike/{bike_id}");
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[get("/{id}/edit")]
pub async fn edit_bike_page(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    model_repo: web::Data<ModelRepository>,
    path: web::Path<(Id,)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    let bike_id = path.into_inner().0;
    let bike: BikeDetail = <BikeRepository as DbReadOne<BikeGetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &BikeGetById::new_admin(bike_id),
    )
    .await?;

    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let template_name = get_template_name(&request, "bike/admin/edit");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeEditTemplate {
        bike,
        models,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/edit")]
pub async fn edit_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    form: web::Form<BikeEditForm>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let book_update = BikeUpdate::new(
        &form.bike_id,
        Some(&form.name),
        Some(&form.model_id),
        None,
        Some(&form.description),
        None,
        None,
        None
    );
    bike_repo.update(&book_update).await?;

    let path = format!("/bike/{}", form.bike_id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[get("/thumbnail/{id}/upload")]
pub async fn upload_bike_thumbnail_page(
    request: HttpRequest,
    bike_repo: web::Data<BikeRepository>,
    identity: Option<Identity>,
    path: web::Path<(Id,)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let bike_id = path.into_inner().0;
    let bike: BikeDetail = <BikeRepository as DbReadOne<BikeGetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &BikeGetById::new_admin(bike_id),
    )
    .await?;
    let template_name = get_template_name(&request, "bike/admin/thumbnail/upload");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeThumbnailUploadTemplate {
        message: String::new(),
        bike: BikeDisplay::from(bike),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/thumbnail/upload")]
pub async fn upload_bike_thumbnail(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    MultipartForm(form): MultipartForm<BikeThumbnailEditForm>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let bike_id = form.bike_id.0;

    let bike: BikeDetail = <BikeRepository as DbReadOne<BikeGetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &BikeGetById::new_admin(bike_id),
    )
    .await?;

    let thumbnail_path = validate_file(&form.thumbnail, Uuid::new_v4(), "image", "thumbnail")?;

    save_file(
        form.thumbnail,
        &thumbnail_path,
        &ImageDimensions::new(THUMBNAIL_SIZE, THUMBNAIL_SIZE),
    )?;

    remove_file(&bike.thumbnail)?;

    bike_repo
        .update(&BikeUpdate::update_thumbnail(bike.id, &thumbnail_path))
        .await?;

    let handler = format!("/bike/{bike_id}");
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, handler))
        .finish())
}
