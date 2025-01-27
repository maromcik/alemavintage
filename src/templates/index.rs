use serde::Serialize;
use crate::database::models::bike::BikeDisplay;
use crate::database::models::image::OtherImage;

#[derive(Serialize)]
pub struct IndexTemplate<'a> {
    pub logged_in: bool,
    pub bikes: &'a Vec<BikeDisplay>,
    pub images: &'a Vec<OtherImage>
}

#[derive(Serialize)]
pub struct AboutTemplate<'a> {
    pub logged_in: bool,
    pub images: &'a Vec<OtherImage>
}
