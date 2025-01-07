use serde::Serialize;
use crate::database::models::bike::BikeDisplay;
use crate::database::models::image::OtherImage;

#[derive(Serialize)]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub bikes: Vec<BikeDisplay>,
    pub images: Vec<OtherImage>
}

#[derive(Serialize)]
pub struct AboutTemplate {
    pub logged_in: bool,
}
