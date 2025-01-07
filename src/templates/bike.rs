use crate::database::models::bike::{BikeDisplay};
use crate::database::models::model::ModelDetail;
use crate::database::models::tag::TagJoin;
use crate::database::models::Id;
use serde::Serialize;
use crate::database::models::image::BikeImage;

#[derive(Serialize)]
pub struct BikesTemplate {
    pub logged_in: bool,
    pub bikes: Vec<BikeDisplay>,
}

#[derive(Serialize)]
pub struct BikeCreateTemplate {
    pub models: Vec<ModelDetail>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BikeEditTemplate {
    pub bike: BikeDisplay,
    pub models: Vec<ModelDetail>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BikeDisplayTemplate {
    pub bike: BikeDisplay,
    pub bike_images: Vec<BikeImage>,
    pub tags: Vec<TagJoin>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BikeUploadFormTemplate {
    pub message: String,
}

#[derive(Serialize)]
pub struct BikeReuploadFormTemplate {
    pub message: String,
    pub bike_id: Id,
}

#[derive(Serialize)]
pub struct BikeThumbnailUploadTemplate {
    pub message: String,
    pub bike_id: Id,
}
