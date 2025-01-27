use crate::database::common::{DbCreate, DbDelete, DbReadMany};
use crate::database::models::image::{
    ImageCreate, OtherImageSearch, OtherImagesCreate,
};
use crate::database::models::{GetById, Id};
use crate::database::repositories::image::repository::ImageRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::forms::image::ImageUploadForm;
use crate::handlers::helpers::{get_template_name, get_user_from_identity};
use crate::templates::image::{ImageUploadFormTemplate, ImagesTemplate};
use crate::utilities::file::remove_file;
use crate::utilities::image::{ImageDimensions, ImageProcessor};
use crate::utils::AppState;
use crate::{authorized, IMAGE_SIZE, THUMBNAIL_SIZE};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[get("")]
pub async fn get_images(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    image_repo: web::Data<ImageRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let images = image_repo.read_many(&OtherImageSearch::new(None)).await?;

    let template_name = get_template_name(&request, "image");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ImagesTemplate {
        logged_in: true,
        images: &images,
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/upload")]
pub async fn upload_images_page(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    image_repo: web::Data<ImageRepository>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;
    
    let image_types = image_repo.read_many(&()).await?;
    
    let template_name = get_template_name(&request, "image/upload");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(ImageUploadFormTemplate {
        message: "",
        image_types: &image_types
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/upload")]
pub async fn upload_images(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    image_repo: web::Data<ImageRepository>,
    MultipartForm(form): MultipartForm<ImageUploadForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let image_type = form.image_type.0;

    let images = form
        .photos
        .into_par_iter()
        .map(|photo| {
            let processor = ImageProcessor::builder(photo).load_image_processor()?;
            let high_res = processor.resize_img(&ImageDimensions::new(IMAGE_SIZE, IMAGE_SIZE))?;
            let thumbnail =
                processor.resize_img(&ImageDimensions::new(THUMBNAIL_SIZE, THUMBNAIL_SIZE))?;
            Ok(ImageCreate::new(
                &high_res.path,
                &high_res.width,
                &high_res.height,
                &thumbnail.path,
            ))
        })
        .collect::<Vec<Result<ImageCreate, AppError>>>();

    let images = images.into_iter().filter_map(Result::ok).collect();
    
    image_repo
        .create(&OtherImagesCreate::new(image_type, images))
        .await?;

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/image"))
        .finish())
}

#[delete("/{id}/delete")]
pub async fn delete_image(
    request: HttpRequest,
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    image_repo: web::Data<ImageRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let _ = get_user_from_identity(u, &user_repo).await?;

    let image_id = path.into_inner().0;

    let images = image_repo.delete(&GetById::new(image_id)).await?;

    for image in images {
        remove_file(&image.path)?;
        remove_file(&image.thumbnail_path)?;
    }

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/image"))
        .finish())
}
