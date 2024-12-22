use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::{
    Bike, BikeCreateSessionKeys, BikeDetail, BikeGetById, BikeImage, BikeImageCreate,
    BikeMetadataForm, BikeUpdate,
};
use crate::database::models::user::{User, UserSearch};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::forms::bike::BikeUploadForm;
use crate::forms::user::EmailForm;
use crate::handlers::utilities::{is_htmx, remove_file, save_file, validate_file, ImageDimensions};
use crate::utils::AppState;
use crate::{IMAGE_SIZE, THUMBNAIL_SIZE};
use actix_identity::Identity;
use actix_multipart::form::tempfile::TempFile;
use actix_session::Session;
use actix_web::{web, HttpRequest};
use lettre::message::Mailbox;
use lettre::{AsyncTransport, Message};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use uuid::Uuid;

pub fn get_template_name(request: &HttpRequest, path: &str) -> String {
    if is_htmx(request) {
        format!("{path}/content.html")
    } else {
        format!("{path}/page.html")
    }
}

pub async fn hard_delete_bike(
    bike_repo: &web::Data<BikeRepository>,
    bike_ids: Vec<Id>,
) -> Result<(), AppError> {
    for bike_id in bike_ids {
        hard_delete_bike_images(bike_repo, bike_id).await?;

        let bikes = <BikeRepository as DbDelete<GetById, Bike>>::delete(
            &bike_repo,
            &GetById::new_with_deleted(bike_id),
        )
        .await?;
        for bike in bikes {
            remove_file(&bike.thumbnail)?;
        }
    }
    Ok(())
}

pub async fn hard_delete_bike_images(
    bike_repo: &web::Data<BikeRepository>,
    bike_id: Id,
) -> Result<(), AppError> {
    let bike_images = <BikeRepository as DbDelete<GetById, BikeImage>>::delete(
        bike_repo,
        &GetById::new_with_deleted(bike_id),
    )
    .await?;

    for image in bike_images {
        remove_file(&image.path)?;
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
    let thumbnail_path = save_bike_thumbnail_helper(form.thumbnail)?;

    let bike_update = BikeUpdate::update_thumbnail_and_mark_complete(bike_id, &thumbnail_path);
    let bikes = bike_repo.update(&bike_update).await?;

    let bike = bikes
        .into_iter()
        .next()
        .ok_or_else(|| AppError::new(AppErrorKind::NotFound, "Bike {bike_id} not found"))?;

    save_bike_images_helper(form.photos, bike_repo, bike.id).await?;

    Ok(bike)
}

pub async fn save_bike_images_helper(
    photos: Vec<TempFile>,
    bike_repo: &web::Data<BikeRepository>,
    bike_id: Id,
) -> Result<(), AppError> {
    let image_dimensions = ImageDimensions::new(IMAGE_SIZE, IMAGE_SIZE);
    let paths = photos
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
        .create(&BikeImageCreate::new(bike_id, paths))
        .await?;
    Ok(())
}

pub fn save_bike_thumbnail_helper(thumbnail: TempFile) -> Result<String, AppError> {
    let thumbnail_path = validate_file(&thumbnail, Uuid::new_v4(), "image", "thumbnail")?;
    save_file(
        thumbnail,
        &thumbnail_path,
        &ImageDimensions::new(THUMBNAIL_SIZE, THUMBNAIL_SIZE),
    )?;
    Ok(thumbnail_path)
}

pub struct Email {
    pub from: Mailbox,
    pub to: Mailbox,
    pub subject: String,
    pub body: String,
}

impl Email {
    pub fn new<'a>(
        form: &'a impl EmailForm<FormField<'a> = &'a str>,
        to: &str,
        bike: Option<&BikeDetail>,
    ) -> Result<Self, AppError> {
        let subject = match bike {
            None => format!("Nová otázka od používateľa {}", form.name()),
            Some(bike) => format!("Nová otázka od {} ohľadom {}", form.name(), bike.name),
        };

        let body = match bike {
            None => {
                format!(
                    "Používateľ {} sa pýta:\n
            \n
            {}\n
            \n
            Kontaktné údaje\n
            email: {}\n
            tel.: {}",
                    form.name(),
                    form.message(),
                    form.from(),
                    form.tel()
                )
            }
            Some(bike) => {
                format!(
                    "Používateľ {} má záujem o bicykel {} ({} {})\n
            \n
            {}\n
            \n
            Kontaktné údaje\n
            email: {}\n
            tel.: {}",
                    form.name(),
                    bike.name,
                    bike.brand_name,
                    bike.model_name,
                    form.message(),
                    form.from(),
                    form.tel()
                )
            }
        };

        Ok(Self {
            from: form.from().parse::<Mailbox>()?,
            to: to.parse::<Mailbox>()?,
            subject,
            body,
        })
    }

    pub fn convert_to_message(self) -> Result<Message, AppError> {
        Message::builder()
            .reply_to(self.from.clone())
            .from(self.from)
            .to(self.to)
            .subject(self.subject)
            .body(self.body)
            .map_err(AppError::from)
    }
}

pub async fn send_emails<'a, T>(
    identity: Option<&Identity>,
    user_repo: &web::Data<UserRepository>,
    bike_repo: &web::Data<BikeRepository>,
    state: &web::Data<AppState>,
    form: &'a T,
) -> Result<(), AppError>
where
    T: EmailForm<FormField<'a> = &'a str>,
{
    let admins = user_repo.read_many(&UserSearch::new_admins_only()).await?;

    let bike = match form.bike_id() {
        None => None,
        Some(bike_id) => {
            let params = BikeGetById::new(bike_id, identity.is_some(), false);
            let bike: BikeDetail =
                <BikeRepository as DbReadOne<BikeGetById, BikeDetail>>::read_one(
                    bike_repo.as_ref(),
                    &params,
                )
                .await?;
            Some(bike)
        }
    };

    for user in admins {
        let email = Email::new(form, &user.email, bike.as_ref())?.convert_to_message()?;
        state.mailer.send(email).await?;
    }
    Ok(())
}
