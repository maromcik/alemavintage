use serde::Serialize;
use crate::database::common::EntityById;
use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable};
use crate::database::models::Id;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct Image {
    pub id: Id,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub thumbnail_path: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct BikeImage {
    pub id: Id,
    pub bike_id: Option<Id>,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub thumbnail_path: String,
}

impl EntityById for BikeImage {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

pub struct BikeImageSearch {
    pub bike_id: Option<Id>,
    pub query_params: DbQueryParams,
}

impl BikeImageSearch {
    pub fn new(bike_id: Option<Id>) -> Self {
        Self {
            bike_id,
            query_params: DbQueryParams::order(
                DbOrderColumn::new_column_only(DbColumn::Id, DbOrder::Asc),
                None,
            ),
        }
    }

    #[allow(dead_code)]
    pub fn with_params(bike_id: Option<Id>, query_params: DbQueryParams) -> Self {
        Self {
            bike_id,
            query_params,
        }
    }

    #[allow(dead_code)]
    pub fn search_by_bike_id(bike_id: Id) -> Self {
        Self {
            bike_id: Some(bike_id),
            query_params: DbQueryParams::order(
                DbOrderColumn::new(DbTable::BikeImage, DbColumn::Path, DbOrder::Asc),
                None,
            ),
        }
    }
}

pub struct ImageCreate {
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub thumbnail_path: String,
}

impl ImageCreate {
    pub fn new(path: &str, width: &i32, height: &i32, thumbnail_path: &String) -> Self {
        Self {
            path: path.to_owned(),
            width: *width,
            height: *height,
            thumbnail_path: thumbnail_path.to_owned(),
        }
    }
}

pub struct BikeImagesCreate {
    pub bike_id: Option<Id>,
    pub bike_images: Vec<ImageCreate>,
}

impl BikeImagesCreate {
    pub fn new(bike_id: Id, paths: Vec<ImageCreate>) -> Self {
        Self {
            bike_id: Some(bike_id),
            bike_images: paths,
        }
    }
}

pub struct BikeImageGetById {
    pub bike_id: Id,
}

impl EntityById for BikeImageGetById {
    fn id(&self) -> Id {
        self.bike_id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

impl BikeImageGetById {
    pub fn new(bike_id: Id) -> Self {
        Self { bike_id }
    }
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct OtherImage {
    pub id: Id,
    pub image_type: Id,
    pub image_type_name: String,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub thumbnail_path: String,
}

pub struct OtherImagesCreate {
    pub image_type: Id,
    pub images: Vec<ImageCreate>,
}

impl OtherImagesCreate {
    pub fn new(image_type: Id, images: Vec<ImageCreate>) -> Self {
        Self { image_type, images }
    }
}

pub struct OtherImageSearch {
    pub image_type: Option<Id>,
}

impl OtherImageSearch {
    pub fn new(image_type: Option<Id>) -> Self {
        Self {
            image_type
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct OtherImageType {
    pub id: Id,
    pub name: String,
}

#[allow(dead_code)]
pub enum OtherImageTypeEnum {
    Homepage,
    About,
    Other
}

pub trait ImageTypeId {
    fn id(&self) -> Id;
}

impl ImageTypeId for OtherImageTypeEnum {
    fn id(&self) -> Id {
        match self {
            OtherImageTypeEnum::Homepage => 1,
            OtherImageTypeEnum::About => 2,
            OtherImageTypeEnum::Other => 3
        }
    }
}