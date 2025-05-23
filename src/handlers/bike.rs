use crate::database::common::query_parameters::{
    DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable,
};
use crate::database::common::repository::DbCreate;
use crate::database::common::{DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::{
    Bike, BikeCreate, BikeCreateSessionKeys, BikeDetail, BikeDetailSessionKeys, BikeDisplay,
    BikeGetById, BikeSearch, BikeUpdate,
};
use crate::database::models::image::BikeImageSearch;
use crate::database::models::model::ModelSearch;
use crate::database::models::tag::{TagJoin, TagSearch};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::image::repository::ImageRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::database::repositories::tag::repository::TagRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::forms::bike::{
    BikeCreateForm, BikeEditForm, BikeImagesEditForm, BikeThumbnailEditForm, BikeUploadForm,
};
use crate::handlers::helpers::{
    create_bike_preview, get_metadata_from_session, get_template_name, get_user_from_identity,
    hard_delete_bike, hard_delete_bike_images, hard_delete_previews, save_bike_images_helper,
    upload_bike_helper,
};
use crate::templates::bike::{
    BikeCreateTemplate, BikeDisplayTemplate, BikeEditTemplate, BikeReuploadFormTemplate,
    BikeThumbnailUploadTemplate, BikeUploadFormTemplate, BikesTemplate,
};
use crate::{authorized, AppState};
use actix_identity::Identity;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use std::io::Write;
use zip::write::{FileOptions, SimpleFileOptions};

#[get("")]
pub async fn get_bikes(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let bikes = bike_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::order(
            DbOrderColumn::new_column_only(DbColumn::EditedAt, DbOrder::Desc),
            identity.is_none().then_some(DbTable::Bike),
        )))
        .await?;

    let template_name = get_template_name(&request, "bike");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikesTemplate {
        logged_in: identity.is_some(),
        bikes: &bikes
            .into_iter()
            .map(|bike| BikeDisplay::from(bike).description_to_markdown())
            .collect(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/{id}")]
pub async fn get_bike_detail(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    image_repo: web::Data<ImageRepository>,
    tag_repo: web::Data<TagRepository>,
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

    let bike_images = image_repo
        .read_many(&BikeImageSearch::new(Some(bike.id)))
        .await?;

    let tags: Vec<TagJoin> = tag_repo
        .read_many(&TagSearch::new(None, None, Some(&bike.id)))
        .await?;

    let template_name = get_template_name(&request, "bike/detail");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeDisplayTemplate {
        bike: &BikeDisplay::from(bike).description_to_markdown(),
        bike_images: &bike_images,
        tags: &tags,
        logged_in: identity.is_some(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/create")]
pub async fn create_bike_page(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    model_repo: web::Data<ModelRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let template_name = get_template_name(&request, "bike/admin/create");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeCreateTemplate {
        models: &models,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/upload")]
pub async fn upload_bike_page(
    request: HttpRequest,
    user_repo: web::Data<UserRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

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

    let bike_create = BikeCreate::new(
        &form.name,
        form.model_id,
        None,
        &form.description,
        &form.year,
        &((form.price * 100_f64).round() as i32),
        &form.height,
        &form.top_tube_size,
        &form.frame,
        &form.seat_tube_sizes,
        &form.headset,
        &form.crankset,
        &form.bottom_bracket,
        &form.front_derail,
        &form.rear_derail,
        &form.brakes,
        &form.shifters,
        &form.brake_levers,
        &form.saddle,
        &form.seat_post,
        &form.hubs,
        &form.rims,
        &form.handlebar,
        &form.stem,
    );

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
    image_repo: web::Data<ImageRepository>,
    MultipartForm(form): MultipartForm<BikeUploadForm>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);
    let metadata = get_metadata_from_session(&session, &session_keys)?;
    let bike_id = metadata.bike_id;

    let bike = match upload_bike_helper(bike_id, &bike_repo, &image_repo, form.thumbnail).await {
        Ok(bike) => bike,
        Err(err) => {
            let template_name = get_template_name(&request, "bike/admin/upload");
            let env = state.jinja.acquire_env()?;
            let template = env.get_template(&template_name)?;
            let body = template.render(BikeUploadFormTemplate {
                message: err.message,
            })?;
            return Ok(HttpResponse::Ok().content_type("text/html").body(body));
        }
    };

    tokio::task::spawn(async move {
        save_bike_images_helper(form.photos, &bike_repo, &image_repo, bike.id).await
    });

    session.remove(session_keys.bike_id.as_str());

    let url = format!("/bike/{bike_id}",);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, url))
        .finish())
}

#[delete("/{id}/hide")]
pub async fn hide_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let bike_id = path.into_inner().0;
    bike_repo.hide(&GetById::new_with_deleted(bike_id)).await?;

    let path = format!("/bike/{bike_id}");
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[delete("/{id}/delete")]
pub async fn remove_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    image_repo: web::Data<ImageRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let bike_id = path.into_inner().0;

    hard_delete_bike(&bike_repo, &image_repo, vec![bike_id]).await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/bike"))
        .finish())
}

#[put("/{id}/restore")]
pub async fn restore_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

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
    user_repo: web::Data<UserRepository>,
    model_repo: web::Data<ModelRepository>,
    path: web::Path<(Id,)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

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
        bike: &BikeDisplay::from(bike),
        models: &models,
        logged_in: true,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/edit")]
pub async fn edit_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    user_repo: web::Data<UserRepository>,
    form: web::Form<BikeEditForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let book_update = BikeUpdate::new(
        &form.bike_id,
        Some(&form.name),
        Some(&form.model_id),
        None,
        Some(&form.description),
        None,
        None,
        None,
        Some(&form.year),
        Some(&((form.price * 100_f64).round() as i32)),
        Some(&form.height),
        Some(&form.top_tube_size),
        Some(&form.frame),
        Some(&form.seat_tube_sizes),
        Some(&form.headset),
        Some(&form.crankset),
        Some(&form.bottom_bracket),
        Some(&form.front_derail),
        Some(&form.rear_derail),
        Some(&form.brakes),
        Some(&form.shifters),
        Some(&form.brake_levers),
        Some(&form.saddle),
        Some(&form.seat_post),
        Some(&form.hubs),
        Some(&form.rims),
        Some(&form.handlebar),
        Some(&form.stem),
        None,
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
    user_repo: web::Data<UserRepository>,
    identity: Option<Identity>,
    path: web::Path<(Id,)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let bike_id = path.into_inner().0;

    let template_name = get_template_name(&request, "bike/admin/thumbnail/upload");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeThumbnailUploadTemplate {
        message: String::new(),
        bike_id,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/thumbnail/upload")]
pub async fn upload_bike_thumbnail(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    image_repo: web::Data<ImageRepository>,
    user_repo: web::Data<UserRepository>,
    MultipartForm(form): MultipartForm<BikeThumbnailEditForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let bike_id = form.bike_id.0;

    let bike: Bike = <BikeRepository as DbReadOne<GetById, Bike>>::read_one(
        bike_repo.as_ref(),
        &GetById::new_with_deleted(bike_id),
    )
    .await?;

    hard_delete_previews(&image_repo, vec![bike]).await?;

    let bike_image = create_bike_preview(form.thumbnail, &image_repo).await?;

    bike_repo
        .update(&BikeUpdate::update_thumbnail(bike_id, bike_image.id))
        .await?;

    let handler = format!("/bike/{bike_id}");
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, handler))
        .finish())
}

#[get("/{id}/reupload")]
pub async fn reupload_bike_page(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    path: web::Path<(Id,)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let template_name = get_template_name(&request, "bike/admin/reupload");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikeReuploadFormTemplate {
        message: String::new(),
        bike_id: path.into_inner().0,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/reupload")]
pub async fn reupload_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    image_repo: web::Data<ImageRepository>,
    user_repo: web::Data<UserRepository>,
    MultipartForm(form): MultipartForm<BikeImagesEditForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let bike_id = form.bike_id.0;

    if form.delete_existing.unwrap_or(Text(false)).0 {
        hard_delete_bike_images(&image_repo, bike_id).await?;
    }

    tokio::task::spawn(async move {
        save_bike_images_helper(form.photos, &bike_repo, &image_repo, bike_id).await
    });

    let url = format!("/bike/{}", bike_id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, url))
        .finish())
}

#[get("/{id}/clone")]
pub async fn clone_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    user_repo: web::Data<UserRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let bike = bike_repo
        .make_clone(&GetById::new(path.into_inner().0))
        .await?;

    let url = format!("/bike/{}", bike.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, url))
        .finish())
}

#[get("/{id}/download")]
pub async fn download_bike_images(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    user_repo: web::Data<UserRepository>,
    image_repo: web::Data<ImageRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let bike: BikeDetail = <BikeRepository as DbReadOne<BikeGetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &BikeGetById::new(path.into_inner().0, false, false),
    )
    .await?;

    let bike_images = image_repo
        .read_many(&BikeImageSearch::new(Some(bike.id)))
        .await?;
    
    let buf = Vec::new();
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(buf));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for (idx, img) in bike_images.iter().enumerate() {
        let file_contents = tokio::fs::read(format!(".{}", img.path)).await?;
        zip.start_file(format!("bike_image_{idx}.jpg"), options)?;
        zip.write_all(&file_contents)?;
    }
    let zip_contents = zip.finish()?.into_inner();

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .insert_header((
            "Content-Disposition",
            format!("attachment; filename=\"bike_{}_images.zip\"", bike.id),
        ))
        .body(zip_contents))
}
