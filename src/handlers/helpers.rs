use crate::database::common::{DbCreate, DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::{Bike, BikeCreateSessionKeys, BikeDetail, BikeGetById, BikeMetadataForm, BikeUpdate};
use crate::database::models::image::{BikeImageGetById, BikeImagesCreate, Image, ImageCreate};
use crate::database::models::user::{User, UserSearch};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::{AppError, AppErrorKind};
use crate::forms::user::EmailForm;
use crate::handlers::utilities::is_htmx;
use crate::utilities::file::remove_file;
use crate::utilities::image::{AppImage, ImageDimensions, ImageProcessor};
use crate::utils::AppState;
use crate::{IMAGE_SIZE, LOW_IMAGE_SIZE, THUMBNAIL_SIZE};
use actix_identity::Identity;
use actix_multipart::form::tempfile::TempFile;
use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use lettre::message::Mailbox;
use lettre::{AsyncTransport, Message};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::Arc;
use crate::database::repositories::image::repository::ImageRepository;

pub fn get_template_name(request: &HttpRequest, path: &str) -> String {
    if is_htmx(request) {
        format!("{path}/content.html")
    } else {
        format!("{path}/page.html")
    }
}

pub async fn hard_delete_bike(
    bike_repo: &web::Data<BikeRepository>,
    image_repo: &web::Data<ImageRepository>,
    bike_ids: Vec<Id>,
) -> Result<(), AppError> {
    for bike_id in bike_ids {
        hard_delete_bike_images(image_repo, bike_id).await?;

        let bikes = <BikeRepository as DbDelete<GetById, Bike>>::delete(
            bike_repo,
            &GetById::new_with_deleted(bike_id),
        )
        .await?;
        
        hard_delete_previews(image_repo, bikes).await?;
    }
    Ok(())
}

pub async fn hard_delete_previews(
    image_repo: &web::Data<ImageRepository>,
    bikes: Vec<Bike>,
)-> Result<(), AppError>{
    for bike in bikes {
        if let Some(preview_id) = bike.preview {
            hard_delete_preview(image_repo, preview_id).await?;
        }
    }
    Ok(())
}

pub async fn hard_delete_preview(
    image_repo: &web::Data<ImageRepository>,
    preview_id: Id,
) -> Result<(), AppError> {
    let previews = <ImageRepository as DbDelete<GetById, Image>>::delete(
        image_repo,
        &GetById::new(preview_id),
    )
    .await?;
    for preview in previews {
        remove_file(&preview.path)?;
        remove_file(&preview.thumbnail_path)?;
    }
    Ok(())
}

pub async fn hard_delete_bike_images(
    image_repo: &web::Data<ImageRepository>,
    bike_id: Id,
) -> Result<(), AppError> {
    let bike_images = <ImageRepository as DbDelete<BikeImageGetById, Image>>::delete(
        image_repo,
        &BikeImageGetById::new(bike_id),
    )
    .await?;

    for image in bike_images {
        remove_file(&image.path)?;
        remove_file(&image.thumbnail_path)?;
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

pub async fn create_bike_preview(
    file: TempFile,
    image_repo: &web::Data<ImageRepository>,
) -> Result<Image, AppError> {
    let (preview, thumbnail) = save_bike_thumbnail_helper(file)?;

    match image_repo
        .create(&ImageCreate::new(
            &preview.path,
            &preview.width,
            &preview.height,
            &thumbnail.path,
        ))
        .await
    {
        Ok(bike_image) => Ok(bike_image),
        Err(e) => {
            remove_file(&preview.path)?;
            remove_file(&thumbnail.path)?;
            Err(AppError::from(e))
        }
    }
}

pub async fn upload_bike_helper(
    bike_id: Id,
    bike_repo: &web::Data<BikeRepository>,
    image_repo: &web::Data<ImageRepository>,
    thumbnail: TempFile,
) -> Result<Bike, AppError> {
    let bike_image = create_bike_preview(thumbnail, image_repo).await?;
    let bike_update = BikeUpdate::update_thumbnail_and_mark_complete(bike_id, bike_image.id);
    let bikes = bike_repo.update(&bike_update).await?;

    let bike = bikes
        .into_iter()
        .next()
        .ok_or_else(|| AppError::new(AppErrorKind::NotFound, "Bike {bike_id} not found"))?;

    Ok(bike)
}

pub async fn save_bike_images_helper(
    photos: Vec<TempFile>,
    bike_repo: &web::Data<BikeRepository>,
    image_repo: &web::Data<ImageRepository>,
    bike_id: Id,
) -> Result<(), AppError> {
    let image_dimensions = ImageDimensions::new(IMAGE_SIZE, IMAGE_SIZE);
    let thumbnail_image_dimensions = ImageDimensions::new(LOW_IMAGE_SIZE, LOW_IMAGE_SIZE);

    bike_repo
        .update(&BikeUpdate::update_status(
            bike_id,
            "<p class=\"text-blue-500\">PROCESSING IMAGES</p>",
        ))
        .await?;

    let paths = tokio::task::spawn_blocking(move || {
        photos
            .into_par_iter()
            .map(|photo| {
                let processor = ImageProcessor::builder(photo).load_image_processor()?;
                let high_res = processor.resize_img(&image_dimensions)?;
                let thumbnail = processor.resize_img(&thumbnail_image_dimensions)?;
                Ok(ImageCreate::new(
                    &high_res.path,
                    &high_res.width,
                    &high_res.height,
                    &thumbnail.path,
                ))
            })
            .collect::<Vec<Result<ImageCreate, AppError>>>()
    })
    .await?;

    let (bike_images, errors): (Vec<ImageCreate>, Vec<String>) =
        paths
            .into_iter()
            .fold((vec![], vec![]), |(mut correct, mut errored), path| {
                match path {
                    Ok(p) => correct.push(p),
                    Err(e) => errored.push(e.message),
                }
                (correct, errored)
            });

    image_repo
        .create(&BikeImagesCreate::new(bike_id, bike_images))
        .await?;

    let mut errors = errors
        .iter()
        .map(|e| format!("<p class=\"text-error\">{e}</p>"))
        .collect::<Vec<String>>();

    if errors.is_empty() {
        errors.push("<p class=\"text-success\">OK</p>".to_string());
    }

    bike_repo
        .update(&BikeUpdate::update_status(
            bike_id,
            errors.join("\n").as_str(),
        ))
        .await?;

    Ok(())
}

pub fn save_bike_thumbnail_helper(thumbnail: TempFile) -> Result<(AppImage, AppImage), AppError> {
    let processor = ImageProcessor::builder(thumbnail).load_image_processor()?;
    let preview = processor.resize_img(&ImageDimensions::new(IMAGE_SIZE, IMAGE_SIZE))?;
    let thumbnail = processor.resize_img(&ImageDimensions::new(THUMBNAIL_SIZE, THUMBNAIL_SIZE))?;
    Ok((preview, thumbnail))
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
        domain: &Arc<String>,
        bike: Option<&BikeDetail>,
    ) -> Result<Self, AppError> {
        let subject = match bike {
            None => format!("Nová otázka od používateľa {}", form.name()),
            Some(bike) => format!("Nová otázka od {} ohľadom {}", form.name(), bike.name),
        };

        let mut body = match bike {
            None => {
                format!("Používateľ {} sa pýta:\n", form.name(),)
            }
            Some(bike) => {
                format!(
                    "Používateľ {} má záujem o bicykel {} ({}/bike/{})\n",
                    form.name(),
                    bike.name,
                    domain,
                    bike.id,
                )
            }
        };

        body.push_str(&format!(
            "
Správa:

{}
    
Kontaktné údaje
Email: {}
Tel.: {}
Krajina: {}
Mesto: {}
Adresa: {}
            ",
            form.message(),
            form.from(),
            form.tel(),
            form.country(),
            form.city(),
            form.address()
        ));

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
        let email =
            Email::new(form, &user.email, &state.domain, bike.as_ref())?.convert_to_message()?;
        state.mailer.send(email).await?;
    }
    Ok(())
}

pub async fn contact_admin_helper<'a, T>(
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    identity: Option<Identity>,
    state: web::Data<AppState>,
    form: &'a T,
) -> Result<HttpResponse, AppError>
where
    T: EmailForm<FormField<'a> = &'a str>,
{
    match send_emails(identity.as_ref(), &user_repo, &bike_repo, &state, form).await {
        Ok(()) => Ok(HttpResponse::Ok().content_type("text/html").body("Sent")),
        Err(err) => match err.app_error_kind {
            AppErrorKind::EmailAddressError => Ok(HttpResponse::BadRequest()
                .content_type("text/html")
                .body("Invalid Email Address")),
            _ => Err(err),
        },
    }
}
