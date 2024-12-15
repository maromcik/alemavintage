use askama::Template;
use crate::database::models::bike::{BikeDetail, BikeImage};
use crate::database::models::brand::Brand;
use crate::database::models::model::Model;

#[derive(Template)]
#[template(path = "studio_create_bike.html")]
pub struct BikeCreatePageTemplate {
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
}

#[derive(Template)]
#[template(path = "bike/bike_create.html")]
pub struct BikeCreateContentTemplate {
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
}

pub struct BikeDetailBase {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>
}


#[derive(Template)]
#[template(path = "detail.html")]
pub struct BikeDetailPageTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>
}

#[derive(Template)]
#[template(path = "bike/detail-content.html")]
pub struct BikeDetailContentTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>
}

impl From<BikeDetailBase> for BikeDetailPageTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images
        }
    }
}

impl From<BikeDetailBase> for BikeDetailContentTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images
        }
    }
}


#[derive(Template)]
#[template(path = "detail_admin.html")]
pub struct BikeDetailAdminPageTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>
}

#[derive(Template)]
#[template(path = "bike/detail_admin-content.html")]
pub struct BikeDetailAdminContentTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>
}

impl From<BikeDetailBase> for BikeDetailAdminPageTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images,
        }
    }
}

impl From<BikeDetailBase> for BikeDetailAdminContentTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images,
        }
    }
}

#[derive(Template)]
#[template(path = "bike/bike_upload.html")]
pub struct BikeUploadFormTemplate {
    pub message: String,
}