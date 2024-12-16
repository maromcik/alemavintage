use crate::database::models::bike::BikeDisplay;
use crate::database::models::brand::Brand;
use crate::database::models::model::Model;
use serde::Serialize;


#[derive(Serialize)]
pub struct BikesTemplate {
    pub logged_in: bool,
    pub bikes: Vec<BikeDisplay>,
}


#[derive(Serialize)]
pub struct BikeCreateTemplate {
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BikeEditTemplate {
    pub bike: BikeDisplay,
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
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

