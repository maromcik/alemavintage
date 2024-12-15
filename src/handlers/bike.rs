use actix_web::http::header::LOCATION;
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use crate::database::models::Id;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::helpers::get_bike_detail_base;
use crate::templates::bike::{BikeDetailContentTemplate, BikeDetailPageTemplate, BikeUploadFormTemplate};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use askama::Template;
use uuid::Uuid;
use crate::{authorized, parse_host};
use crate::database::models::bike::BikeCreate;
use crate::database::repositories::user::repository::UserRepository;
use crate::forms::bike::BikeUploadForm;
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



#[post("/upload")]
pub async fn upload_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    session: Session,
    user_repo: web::Data<UserRepository>,
    bike_repo: web::Data<BikeRepository>,
    MultipartForm(mut form): MultipartForm<BikeUploadForm>,
) -> Result<HttpResponse, AppError> {
    let uuid = Uuid::new_v4();
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);
    
    let thumbnail_path = validate_file(&form.thumbnail, uuid, "image", "bike")?;
    save_file(form.thumbnail, &thumbnail_path)?;
    
    for photo in form.photos {
        let path = validate_file(&photo, uuid, "image", "bike")?;
        save_file(photo, &path)?;
    }
    // let metadata = get_metadata_from_session(&session, &session_keys)?;

    // let audio_file = form.audio_file.file.as_file_mut();
    // let lofty_audio_file = match lofty::read_from(audio_file) {
    //     Ok(f) => f,
    //     Err(e) => {
    //         let template = bikeUploadFormTemplate {
    //             message: e.to_string(),
    //         }
    //             .render()?;
    //         return Ok(HttpResponse::Ok().content_type("text/html").body(template));
    //     }
    // };
    // let properties = lofty_audio_file.properties();
    // let length = properties.duration().as_secs_f64();
    
    
    // let book_crate = BikeCreate::new(
    //     &metadata.name,
    //     &user.id,
    //     &metadata.genre_id,
    //     &bike_path,
    //     &length,
    //     thumbnail_path.clone(),
    //     &metadata.description,
    // );
    // let book = bike_repo.create(&book_crate).await?;

    // let genre = genre_repo
    //     .read_one(&GenreGetById::new(&book.genre_id))
    //     .await?;
    
    
    session.remove(session_keys.name.as_str());
    session.remove(session_keys.description.as_str());
    session.remove(session_keys.brand_id.as_str());
    session.remove(session_keys.model_id.as_str());

    // let handler = format!("/bike/{}/manage-content", book.id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, '/'.to_string()))
        .finish())
}