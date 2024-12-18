use actix_identity::Identity;
use actix_session::Session;
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::bike::{BikeCreateSessionKeys, BikeImageSearch, BikeMetadataForm};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::{AppError, AppErrorKind};
use crate::handlers::utilities::{is_htmx, remove_file};
use actix_web::{web, HttpRequest};
use crate::database::models::user::User;
use crate::database::repositories::user::repository::UserRepository;

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

pub fn parse_user_id(identity: Identity) -> Result<Id, AppError> {
    Ok(identity.id()?.parse::<i64>()?)
}

pub fn get_metadata_from_session(
    session: &Session,
    session_keys: &BikeCreateSessionKeys,
) -> Result<BikeMetadataForm, AppError> {
    let Some(name) = session.get::<String>(session_keys.name.as_str())? else {
        return Err(AppError::new(
            AppErrorKind::NotFound,
            "New bike could not be found in the active session",
        ));
    };

    let Some(model_id) = session.get::<i64>(session_keys.model_id.as_str())? else {
        return Err(AppError::new(
            AppErrorKind::NotFound,
            "New bike could not be found in the active session",
        ));
    };

    let Some(description) = session.get::<String>(session_keys.description.as_str())? else {
        return Err(AppError::new(
            AppErrorKind::NotFound,
            "New bike could not be found in the active session",
        ));
    };

    Ok(BikeMetadataForm {
        name,
        description,
        model_id,
    })
}

pub async fn get_user_from_identity(
    identity: Identity,
    user_repo: &web::Data<UserRepository>,
) -> Result<User, AppError> {
    Ok(user_repo
        .read_one(&GetById::new(parse_user_id(identity)?))
        .await?)
}