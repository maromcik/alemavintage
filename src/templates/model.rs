use crate::database::models::bike::{BikeDetail, BikeDisplay};
use crate::database::models::brand::Brand;
use crate::database::models::model::{ModelDetail, ModelDisplay};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ModelCreateTemplate {
    pub brands: Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelEditTemplate {
    pub model: ModelDetail,
    pub brands: Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelTemplate {
    pub models: HashMap<String, Vec<ModelDetail>>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelContentTemplate {
    pub models: Vec<ModelDetail>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelDetailTemplate {
    pub model: ModelDisplay,
    pub bikes: Vec<BikeDisplay>,
    pub logged_in: bool,
}
