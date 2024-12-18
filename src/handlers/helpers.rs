use crate::database::common::DbReadMany;
use crate::database::models::bike::BikeImageSearch;
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::handlers::utilities::{is_htmx, remove_file};
use actix_web::{web, HttpRequest};

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
