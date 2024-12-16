use crate::{authorized, AppState};
use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable};
use crate::database::common::repository::DbCreate;
use crate::database::common::{DbDelete, DbReadMany, DbReadOne, DbUpdate};
use crate::database::models::bike::{BikeCreate, BikeDetail, BikeImageCreate, BikeImageSearch, BikeSearch, BikeUpdate};
use crate::database::models::brand::BrandSearch;
use crate::database::models::model::ModelSearch;
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::forms::bike::{BikeCreateForm, BikeEditForm, BikeUploadForm};
use crate::handlers::helpers::get_bike_detail_base;
use crate::handlers::utilities::{
    get_metadata_from_session, get_user_from_identity, is_htmx, remove_file, save_file,
    validate_file, BikeCreateSessionKeys, ImageDimensions,
};
use crate::templates::bike::{BikeBase, BikeContentTemplate, BikeCreateContentTemplate, BikeCreatePageTemplate, BikeDetailAdminContentTemplate, BikeDetailAdminPageTemplate, BikeDetailContentTemplate, BikeDetailPageTemplate, BikeEditContentTemplate, BikeEditPageTemplate, BikeTemplate, BikeUploadFormTemplate};
use actix_identity::Identity;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use actix_web::http::header::LOCATION;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use askama::Template;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
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
            DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc),
            Some(DbTable::Bike),
        )))
        .await?;

    let template_name = if is_htmx(request) { "bike/content.html" } else { "bike/page.html" };
    
    let template = BikeBase {
        logged_in: identity.is_some(),
        bikes
    };
    
    let temp = state.jinja.get_template(template_name)?;
    let rendered = temp.render(BikeTemplate::from(template))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/{id}/detail")]
pub async fn get_bike_detail(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let base = get_bike_detail_base(&identity, bike_repo, path.into_inner().0, false).await?;

    let body = match is_htmx(request) {
        true => BikeDetailContentTemplate::from(base).render()?,
        false => BikeDetailPageTemplate::from(base).render()?,
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/create")]
pub async fn create_bike_page(
    request: HttpRequest,
    identity: Option<Identity>,
    brand_repo: web::Data<BrandRepository>,
    model_repo: web::Data<ModelRepository>,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let brands = brand_repo.read_many(&BrandSearch::default()).await?;
    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let body = match is_htmx(request) {
        true => BikeCreateContentTemplate {
            brands,
            models,
            logged_in: true,
        }
        .render()?,
        false => BikeCreatePageTemplate {
            brands,
            models,
            logged_in: true,
        }
        .render()?,
    };

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

#[post("/create")]
pub async fn create_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    session: Session,
    user_repo: web::Data<UserRepository>,
    brand_repo: web::Data<BrandRepository>,
    model_repo: web::Data<ModelRepository>,
    form: web::Form<BikeCreateForm>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);

    let model = model_repo.read_one(&GetById::new(form.model_id)).await?;

    let brand = brand_repo.read_one(&GetById::new(form.brand_id)).await?;

    session.insert(session_keys.name.as_str(), &form.name)?;
    session.insert(session_keys.description.as_str(), &form.description)?;
    session.insert(session_keys.brand_id.as_str(), brand.id)?;
    session.insert(session_keys.model_id.as_str(), model.id)?;

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
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let user = get_user_from_identity(u, &user_repo).await?;
    let session_keys = BikeCreateSessionKeys::new(user.id);

    let metadata = get_metadata_from_session(&session, &session_keys)?;

    let thumbnail_path = validate_file(&form.thumbnail, Uuid::new_v4(), "image", "thumbnail")?;

    let bike_create = BikeCreate::new(
        &metadata.name,
        &metadata.brand_id,
        &metadata.model_id,
        &thumbnail_path,
        &metadata.description,
    );

    save_file(
        form.thumbnail,
        &thumbnail_path,
        &ImageDimensions::new(300, 300),
    )?;

    let image_dimensions = ImageDimensions::new(2000, 2000);

    let bike = bike_repo.create(&bike_create).await?;
    let paths = form
        .photos
        .into_par_iter()
        .map(|photo| {
            let path = validate_file(&photo, Uuid::new_v4(), "image", "bike")?;
            save_file(photo, &path, &image_dimensions)?;
            Ok(path)
        })
        .collect::<Result<Vec<String>, AppError>>()?;
    bike_repo
        .create(&BikeImageCreate::new(bike.id, paths))
        .await?;

    session.remove(session_keys.name.as_str());
    session.remove(session_keys.description.as_str());
    session.remove(session_keys.brand_id.as_str());
    session.remove(session_keys.model_id.as_str());

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/studio".to_string()))
        .finish())
}

#[get("/{id}/manage")]
pub async fn manage_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());

    let base = get_bike_detail_base(&Some(u), bike_repo, path.into_inner().0, true).await?;

    let body = match is_htmx(request) {
        true => BikeDetailAdminContentTemplate::from(base).render()?,
        false => BikeDetailAdminPageTemplate::from(base).render()?,
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[delete("/{id}/delete")]
pub async fn remove_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let bike_id = path.into_inner().0;
    bike_repo
        .delete(&GetById::new_with_deleted(bike_id))
        .await?;

    let path = format!("/bike/{}/manage", bike_id);
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
    let u = authorized!(identity, request.path());
    let bike_id = path.into_inner().0;

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

    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, "/studio"))
        .finish())
}

#[put("/{id}/restore")]
pub async fn restore_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let u = authorized!(identity, request.path());
    let bike_id = path.into_inner().0;
    bike_repo
        .restore(&GetById::new_with_deleted(bike_id))
        .await?;
    let path = format!("/bike/{}/manage", bike_id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}

#[get("/{id}/edit")]
pub async fn edit_bike_page(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    brand_repo: web::Data<BrandRepository>,
    model_repo: web::Data<ModelRepository>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let _ = authorized!(identity, request.path());
    let bike_id = path.into_inner().0;
    let bike: BikeDetail = <BikeRepository as DbReadOne<GetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &GetById::new_with_deleted(bike_id),
    )
    .await?;

    let brands = brand_repo.read_many(&BrandSearch::default()).await?;
    let models = model_repo.read_many(&ModelSearch::default()).await?;

    let body = match is_htmx(request) {
        true => BikeEditContentTemplate {
            bike,
            brands,
            models,
            logged_in: true,
        }
        .render()?,
        false => BikeEditPageTemplate {
            bike,
            brands,
            models,
            logged_in: true,
        }
        .render()?,
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("/edit")]
pub async fn edit_bike(
    request: HttpRequest,
    identity: Option<Identity>,
    bike_repo: web::Data<BikeRepository>,
    form: web::Form<BikeEditForm>,
) -> Result<HttpResponse, AppError> {
    let _ = authorized!(identity, request.path());

    let book_update = BikeUpdate::new(
        &form.bike_id,
        Some(&form.name),
        Some(&form.brand_id),
        Some(&form.model_id),
        None,
        Some(&form.description),
        None,
        None
    );
    bike_repo.update(&book_update).await?;

    let path = format!("/bike/{}/manage", form.bike_id);
    Ok(HttpResponse::SeeOther()
        .insert_header((LOCATION, path))
        .finish())
}
