use crate::database::models::bike::BikeDisplay;
use crate::database::models::brand::Brand;
use crate::database::models::model::{ModelDetail, ModelDisplay};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ModelCreateTemplate<'a> {
    pub brands: &'a Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelEditTemplate<'a> {
    pub model: &'a ModelDetail,
    pub brands: &'a Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelTemplate<'a> {
    pub models: &'a HashMap<String, Vec<ModelDetail>>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelContentTemplate<'a> {
    pub models: &'a Vec<ModelDetail>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct ModelDetailTemplate<'a> {
    pub model: &'a ModelDisplay,
    pub bikes: &'a Vec<BikeDisplay>,
    pub logged_in: bool,
}
