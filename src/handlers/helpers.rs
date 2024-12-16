use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::bike::{BikeDetail, BikeDisplay, BikeImageSearch};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::utilities::is_htmx;
use crate::templates::bike::BikeDisplayTemplate;
use actix_identity::Identity;
use actix_web::{web, HttpRequest};
use std::fmt::Display;

pub async fn get_bike_detail_base(
    identity: Option<&Identity>,
    bike_repo: web::Data<BikeRepository>,
    bike_id: Id,
    fetch_deleted: bool,
) -> Result<BikeDisplayTemplate, AppError> {
    let bike: BikeDetail = <BikeRepository as DbReadOne<GetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &GetById {
            id: bike_id,
            fetch_deleted,
        },
    )
    .await?;

    let bike_images: Vec<String> = bike_repo
        .read_many(&BikeImageSearch::new(Some(bike.id)))
        .await?
        .into_iter()
        .map(|image| image.path)
        .collect();
    Ok(BikeDisplayTemplate {
        bike: BikeDisplay::from(bike),
        bike_images,
        logged_in: identity.is_some(),
    })
}


pub fn get_template_name(
    request: &HttpRequest,
    path: &str,
) -> String {
    if is_htmx(request) {
        format!("{path}/content.html")
    } else {
        format!("{path}/page.html")
    }
}
