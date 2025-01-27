use crate::database::common::{DbCreate, DbDelete, DbReadMany};
use crate::database::models::bike::{BikeDisplay, BikeSearch};
use crate::database::models::tag::{TagAssign, TagCreate, TagSearch};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::tag::repository::TagRepository;
use crate::error::AppError;
use crate::forms::tag::TagsAssignForm;
use crate::handlers::helpers::get_template_name;
use crate::templates::bike::BikesTemplate;
use crate::templates::tag::TagsTemplate;
use crate::utils::AppState;
use actix_identity::Identity;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};

#[get("/{id}")]
pub async fn get_tag(
    request: HttpRequest,
    bike_repo: web::Data<BikeRepository>,
    state: web::Data<AppState>,
    identity: Option<Identity>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let bikes = bike_repo
        .read_many(&BikeSearch::search_by_tag_id(path.into_inner().0))
        .await?;

    let template_name = get_template_name(&request, "bike");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(BikesTemplate {
        logged_in: identity.is_some(),
        bikes: &bikes.into_iter().map(|bike| BikeDisplay::from(bike).description_to_markdown()).collect(),
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[post("{tag}")]
pub async fn create_tag(
    request: HttpRequest,
    tag_repo: web::Data<TagRepository>,
    state: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, AppError> {
    let tag = path.into_inner().0;
    tag_repo.create(&TagCreate::new(tag.as_str())).await?;
    let tags = tag_repo.read_many(&TagSearch::default()).await?;
    let template_name = get_template_name(&request, "tag");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(TagsTemplate {
        tags: &tags
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[delete("{id}")]
pub async fn delete_tag(
    request: HttpRequest,
    tag_repo: web::Data<TagRepository>,
    state: web::Data<AppState>,
    path: web::Path<(Id,)>,
) -> Result<HttpResponse, AppError> {
    let tag_id = path.into_inner().0;
    tag_repo.delete(&GetById::new(tag_id)).await?;
    let tags = tag_repo.read_many(&TagSearch::default()).await?;
    let template_name = get_template_name(&request, "tag");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(TagsTemplate {
        tags: &tags
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[put("assign")]
pub async fn assign_tags(
    request: HttpRequest,
    tag_repo: web::Data<TagRepository>,
    state: web::Data<AppState>,
    form: web::Form<TagsAssignForm>,
) -> Result<HttpResponse, AppError> {
    
    tag_repo.create(&TagAssign::new(&form.tags, &form.bike_id)).await?;
    
    let tags = tag_repo.read_many(&TagSearch::default()).await?;
    let template_name = get_template_name(&request, "tag");
    let env = state.jinja.acquire_env()?;
    let template = env.get_template(&template_name)?;
    let body = template.render(TagsTemplate {
        tags: &tags
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}