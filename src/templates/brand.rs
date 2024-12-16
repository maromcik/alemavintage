use crate::database::models::brand::Brand;
use serde::Serialize;


#[derive(Serialize)]
pub struct BrandCreateTemplate {
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
