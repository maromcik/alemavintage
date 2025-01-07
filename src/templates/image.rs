use serde::Serialize;
use crate::database::models::image::{OtherImage, OtherImageType};

#[derive(Serialize)]
pub struct ImagesTemplate {
    pub logged_in: bool,
    pub images: Vec<OtherImage>
}

#[derive(Serialize)]
pub struct ImageUploadFormTemplate {
    pub message: String,
    pub image_types: Vec<OtherImageType>,
}

