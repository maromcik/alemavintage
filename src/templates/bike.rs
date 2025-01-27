use crate::database::models::bike::{BikeDisplay};
use crate::database::models::model::ModelDetail;
use crate::database::models::tag::TagJoin;
use crate::database::models::Id;
use serde::Serialize;
use crate::database::models::image::BikeImage;

#[derive(Serialize)]
pub struct BikesTemplate<'a> {
    pub logged_in: bool,
    pub bikes: &'a Vec<BikeDisplay>,
}

#[derive(Serialize)]
pub struct BikeCreateTemplate<'a> {
    pub models: &'a Vec<ModelDetail>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BikeEditTemplate<'a> {
    pub bike: &'a BikeDisplay,
    pub models: &'a Vec<ModelDetail>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BikeDisplayTemplate<'a> {
    pub bike: &'a BikeDisplay,
    pub bike_images: &'a Vec<BikeImage>,
    pub tags: &'a Vec<TagJoin>,
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
