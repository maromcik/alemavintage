use crate::database::common::{DbCreate, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::{
    Bike, BikeCreateSessionKeys, BikeImageCreate, BikeImageSearch,
    BikeMetadataForm, BikeUpdate,
};
use crate::database::models::user::User;
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::forms::bike::BikeUploadForm;
use crate::handlers::utilities::{is_htmx, remove_file, save_file, validate_file, ImageDimensions};
use actix_identity::Identity;
use actix_session::Session;
use actix_web::{web, HttpRequest};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use uuid::Uuid;
use crate::{IMAGE_SIZE, THUMBNAIL_SIZE};

pub fn get_template_name(request: &HttpRequest, path: &str) -> String {
    if is_htmx(request) {
        format!("{path}/content.html")
    } else {
        format!("{path}/page.html")
    }
}

pub async fn bike_hard_delete(
    bike_repo: &web::Data<BikeRepository>,
    bike_ids: Vec<Id>,
) -> Result<(), AppError> {
    for bike_id in bike_ids {
        let bike_images = bike_repo
            .read_many(&BikeImageSearch::new(Some(bike_id)))
            .await?;

        for image in bike_images {
            remove_file(&image.path)?;
        }

        let bikes = bike_repo
            .hard_delete(&GetById::new_with_deleted(bike_id))
            .await?;
        for bike in bikes {
            remove_file(&bike.thumbnail)?;
        }
    }
    Ok(())
}

pub fn parse_user_id(identity: &Identity) -> Result<Id, AppError> {
    Ok(identity.id()?.parse::<i64>()?)
}

pub fn get_metadata_from_session(
    session: &Session,
    session_keys: &BikeCreateSessionKeys,
) -> Result<BikeMetadataForm, AppError> {
    let Some(bike_id) = session.get::<i64>(session_keys.bike_id.as_str())? else {
        return Err(AppError::new(
            AppErrorKind::NotFound,
            "Bike ID could not be found in the active session",
        ));
    };

    Ok(BikeMetadataForm { bike_id })
}

pub async fn get_user_from_identity(
    identity: Identity,
    user_repo: &web::Data<UserRepository>,
) -> Result<User, AppError> {
    Ok(user_repo
        .read_one(&GetById::new(parse_user_id(&identity)?))
        .await?)
}

pub async fn upload_bike_helper(
    bike_id: Id,
    bike_repo: &web::Data<BikeRepository>,
    form: BikeUploadForm,
) -> Result<Bike, AppError> {
    let thumbnail_path = validate_file(&form.thumbnail, Uuid::new_v4(), "image", "thumbnail")?;

    let bike_update = BikeUpdate::update_thumbnail(bike_id, &thumbnail_path);
    let bikes = bike_repo.update(&bike_update).await?;

    let bike = bikes
        .into_iter()
        .next()
        .ok_or_else(|| AppError::new(AppErrorKind::NotFound, "Bike {bike_id} not found"))?;

    save_file(
        form.thumbnail,
        &thumbnail_path,
        &ImageDimensions::new(THUMBNAIL_SIZE, THUMBNAIL_SIZE),
    )?;

    let image_dimensions = ImageDimensions::new(IMAGE_SIZE, IMAGE_SIZE);

    let paths = form
        .photos
        .into_par_iter()
        .map(|photo| {
            let path = validate_file(&photo, Uuid::new_v4(), "image", "bike")?;
            if let Err(err) = save_file(photo, &path, &image_dimensions) {
                remove_file(&path)?;
                return Err(err);
            }
            Ok(path)
        })
        .collect::<Result<Vec<String>, AppError>>()?;
    bike_repo
        .create(&BikeImageCreate::new(bike.id, paths))
        .await?;
    Ok(bike)
}
