use crate::database::models::brand::Brand;
use crate::database::models::model::ModelDetail;
use serde::Serialize;

#[derive(Serialize)]
pub struct BrandCreateTemplate {
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandEditTemplate {
    pub brand: Brand,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandTemplate {
    pub brands: Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandContentTemplate {
    pub brands: Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandDetailTemplate {
    pub brand: Brand,
    pub models: Vec<ModelDetail>,
    pub logged_in: bool,
}
