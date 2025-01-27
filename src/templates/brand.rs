use crate::database::models::brand::{Brand, BrandDisplay};
use crate::database::models::model::ModelDetail;
use serde::Serialize;

#[derive(Serialize)]
pub struct BrandCreateTemplate {
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandEditTemplate<'a> {
    pub brand: &'a Brand,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandTemplate<'a> {
    pub brands: &'a Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandContentTemplate<'a> {
    pub brands: &'a Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct BrandDetailTemplate<'a> {
    pub brand: &'a BrandDisplay,
    pub models: &'a Vec<ModelDetail>,
    pub logged_in: bool,
}
