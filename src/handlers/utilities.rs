use crate::database::common::DbReadOne;

use crate::database::models::{GetById, Id};

use crate::error::{AppError, AppErrorKind};
use actix_identity::Identity;
use actix_multipart::form::tempfile::TempFile;
use actix_session::Session;
use actix_web::web;

use crate::database::common::error::{BackendError, BackendErrorKind};
use actix_web::http::header::LOCATION;

use crate::MIN_PASS_LEN;
use uuid::Uuid;
use crate::database::models::bike::BikeMetadataForm;
use crate::database::models::user::User;
use crate::database::repositories::user::repository::UserRepository;

pub struct BikeCreateSessionKeys {
    pub name: String,
    pub description: String,
    pub brand_id: String,
    pub model_id: String,
}

impl BikeCreateSessionKeys {
    pub fn new(user_id: Id) -> Self {
        Self {
            name: format!("bike_create_{}_name", user_id),
            description: format!("bike_create_{}_description", user_id),
            brand_id: format!("bike_create_{}_brand_id", user_id),
            model_id: format!("bike_create_{}_model_id", user_id),
        }
    }
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

    let Some(brand_id) = session.get::<i64>(session_keys.brand_id.as_str())? else {
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
        brand_id,
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

pub fn validate_file(
    file: &TempFile,
    uuid: Uuid,
    mime: &str,
    handler: &str,
) -> Result<String, AppError> {
    let extension = match file.file_name.clone() {
        None => "".to_owned(),
        Some(name) => {
            let split_res = name.split('.');
            let vector = split_res.collect::<Vec<&str>>();
            match vector.last() {
                None => "".to_owned(),
                Some(ext) => ext.to_string(),
            }
        }
    };
    let file_path = format!("/media/{handler}_{uuid}_{mime}.{extension}");

    let Some(file_mime) = &file.content_type else {
        return Err(AppError::new(
            AppErrorKind::FileError,
            format!("No MIME type found for {file_path}").as_str(),
        ));
    };

    if !file_mime
        .to_string()
        .starts_with(format!("{mime}/").as_str())
    {
        return Err(AppError::new(
            AppErrorKind::FileError,
            format!("Invalid content type for {file_path}").as_str(),
        ));
    }
    Ok(file_path)
}

pub fn save_file(file: TempFile, path: &str) -> Result<(), AppError> {
    log::info!("saving file to .{path}");
    let path = format!(".{path}");
    if let Err(e) = file.file.persist(path) {
        return Err(AppError::new(
            AppErrorKind::FileError,
            e.to_string().as_str(),
        ));
    };
    Ok(())
}

pub fn remove_file(path: &str) -> Result<(), AppError> {
    let fs_path = format!(".{path}");
    if !path.is_empty() && std::path::Path::new(&fs_path).exists() {
        std::fs::remove_file(&fs_path)?;
    }
    Ok(())
}

#[macro_export]
macro_rules! authorized {
    ($e:expr, $p:expr) => {{
        match $e {
            None => {
                let path = format!("/user/login?ret={}", $p);
                return Ok(HttpResponse::SeeOther()
                    .insert_header((LOCATION, path))
                    .finish());
            }
            Some(v) => v,
        }
    }};
}

// pub async fn authorized_to_modify(
//     bike_repo: &web::Data<BikeRepository>,
//     user_id: Id,
//     bike_id: Id,
// ) -> Result<Bike, AppError> {
//     let bike = bike_repo
//         .read_one(&BikeGetById::new(&bike_id, true))
//         .await?;
//     is_authorized(user_id, bike.author_id)?;
//     Ok(bike)
// }

// pub async fn authorized_to_modify_join(
//     bike_repo: &web::Data<BikeRepository>,
//     user_id: Id,
//     bike_id: Id,
// ) -> Result<BikeDetail, AppError> {
//     let bike = bike_repo
//         .read_one(&BikeGetByIdJoin::new(user_id, bike_id, true))
//         .await?;
//     is_authorized(user_id, bike.author_id)?;
//     Ok(bike)
// }

pub fn is_authorized(user_id: Id, author_id: Id) -> Result<(), AppError> {
    match user_id == author_id {
        true => Ok(()),
        false => Err(AppError::from(BackendError::new(
            BackendErrorKind::UnauthorizedOperation,
        ))),
    }
}

pub fn validate_password(password: &str) -> bool {
    let (lower, upper, numeric, special) =
        password
            .chars()
            .fold((false, false, false, false), |(l, u, n, s), c| {
                (
                    {
                        match c.is_lowercase() {
                            true => true,
                            false => l,
                        }
                    },
                    {
                        match c.is_uppercase() {
                            true => true,
                            false => u,
                        }
                    },
                    {
                        match c.is_numeric() {
                            true => true,
                            false => n,
                        }
                    },
                    {
                        match !c.is_alphanumeric() {
                            true => true,
                            false => s,
                        }
                    },
                )
            });
    lower && upper && numeric && special && password.len() >= MIN_PASS_LEN
}
