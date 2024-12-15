use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable};
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::bike::{BikeDetail, BikeImageSearch, BikeSearch};
use crate::database::models::{GetById, Id};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::templates::bike::BikeDetailBase;
use crate::templates::index::IndexBase;
use actix_web::web;

pub async fn get_bike_detail_base(
    bike_repo: web::Data<BikeRepository>,
    bike_id: Id,
    fetch_deleted: bool
) -> Result<BikeDetailBase, AppError> {
    let bike: BikeDetail = <BikeRepository as DbReadOne<GetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &GetById {id: bike_id, fetch_deleted },
    )
    .await?;

    let bike_images: Vec<String> = bike_repo
        .read_many(&BikeImageSearch::new(Some(bike.id)))
        .await?
        .into_iter()
        .map(|image| image.path)
        .collect();
    Ok(BikeDetailBase { bike, bike_images })
}

pub async fn get_index_base(bike_repo: web::Data<BikeRepository>) -> Result<IndexBase, AppError> {
    let bikes = bike_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::order(
            DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc), Some(DbTable::Bike)
        )))
        .await?;

    let template = IndexBase {
        logged_in: true,
        bikes,
    };
    Ok(template)
}
