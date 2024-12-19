use crate::database::models::bike::{BikeDetail, BikeDisplay};
use crate::database::models::model::ModelDetail;
use serde::Serialize;
use crate::database::models::Id;

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
    pub bike: BikeDetail,
    pub models: Vec<ModelDetail>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BikeDisplayTemplate {
    pub bike: BikeDisplay,
    pub bike_images: Vec<String>,
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
    pub bike_id: Id
}