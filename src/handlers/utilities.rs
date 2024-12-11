use crate::database::common::DbReadOne;

use crate::database::models::Id;

use crate::error::{AppError, AppErrorKind};
use actix_identity::Identity;
use actix_multipart::form::tempfile::TempFile;
use actix_session::Session;
use actix_web::web;

use crate::database::common::error::{BackendError, BackendErrorKind};

use crate::MIN_PASS_LEN;
use uuid::Uuid;

pub struct AudiobookCreateSessionKeys {
    pub name: String,
    pub description: String,
    pub genre_id: String,
}

impl AudiobookCreateSessionKeys {
    pub fn new(user_id: Id) -> Self {
        Self {
            name: format!("audiobook_create_{}_name", user_id),
            description: format!("audiobook_create_{}_description", user_id),
            genre_id: format!("audiobook_create_{}_genre_id", user_id),
        }
    }
}

pub fn parse_user_id(identity: Identity) -> Result<Id, AppError> {
    Ok(identity.id()?.parse::<i64>()?)
}

// pub fn get_metadata_from_session(
//     session: &Session,
//     session_keys: &AudiobookCreateSessionKeys,
// ) -> Result<AudiobookMetadataForm, AppError> {
//     let Some(name) = session.get::<String>(session_keys.name.as_str())? else {
//         return Err(AppError::new(
//             AppErrorKind::NotFound,
//             "New book could not be found in the active session",
//         ));
//     };
//
//     let Some(genre_id) = session.get::<i64>(session_keys.genre_id.as_str())? else {
//         return Err(AppError::new(
//             AppErrorKind::NotFound,
//             "New book could not be found in the active session",
//         ));
//     };
//
//     let Some(description) = session.get::<String>(session_keys.description.as_str())? else {
//         return Err(AppError::new(
//             AppErrorKind::NotFound,
//             "New book could not be found in the active session",
//         ));
//     };
//
//     Ok(AudiobookMetadataForm {
//         name,
//         description,
//         genre_id,
//     })
// }

// pub async fn get_user_from_identity(
//     identity: Identity,
//     user_repo: &web::Data<UserRepository>,
// ) -> Result<User, AppError> {
//     Ok(user_repo
//         .read_one(&UserGetById::new(&parse_user_id(identity)?))
//         .await?)
// }

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
//     audiobook_repo: &web::Data<AudiobookRepository>,
//     user_id: Id,
//     audiobook_id: Id,
// ) -> Result<Audiobook, AppError> {
//     let audiobook = audiobook_repo
//         .read_one(&AudiobookGetById::new(&audiobook_id, true))
//         .await?;
//     is_authorized(user_id, audiobook.author_id)?;
//     Ok(audiobook)
// }

// pub async fn authorized_to_modify_join(
//     audiobook_repo: &web::Data<AudiobookRepository>,
//     user_id: Id,
//     audiobook_id: Id,
// ) -> Result<AudiobookDetail, AppError> {
//     let audiobook = audiobook_repo
//         .read_one(&AudiobookGetByIdJoin::new(user_id, audiobook_id, true))
//         .await?;
//     is_authorized(user_id, audiobook.author_id)?;
//     Ok(audiobook)
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
