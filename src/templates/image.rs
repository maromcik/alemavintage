use serde::Serialize;
use crate::database::models::image::{OtherImage, OtherImageType};

#[derive(Serialize)]
pub struct ImagesTemplate<'a> {
    pub logged_in: bool,
    pub images: &'a Vec<OtherImage>
}

#[derive(Serialize)]
pub struct ImageUploadFormTemplate<'a> {
    pub message: &'a str,
    pub image_types: &'a Vec<OtherImageType>,
}

