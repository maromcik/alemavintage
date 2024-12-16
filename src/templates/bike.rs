use askama::Template;
use serde::Serialize;
use crate::database::models::bike::{BikeDetail, BikeImage};
use crate::database::models::brand::Brand;
use crate::database::models::model::Model;


#[derive(Template, Serialize)]
#[template(path = "bike/page.html")]
pub struct BikeTemplate {
    pub logged_in: bool,
    pub bikes: Vec<BikeDetail>,
}


#[derive(Template)]
#[template(path = "bike/admin/create/page.html")]
pub struct BikeCreatePageTemplate {
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "bike/admin/create/content.html")]
pub struct BikeCreateContentTemplate {
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "bike/admin/edit/page.html")]
pub struct BikeEditPageTemplate {
    pub bike: BikeDetail,
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "bike/admin/edit/content.html")]
pub struct BikeEditContentTemplate {
    pub bike: BikeDetail,
    pub brands: Vec<Brand>,
    pub models: Vec<Model>,
    pub logged_in: bool,
}

pub struct BikeDetailBase {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>,
    pub logged_in: bool,
}


#[derive(Template)]
#[template(path = "bike/detail/page.html")]
pub struct BikeDetailPageTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "bike/detail/content.html")]
pub struct BikeDetailContentTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>,
    pub logged_in: bool,
}

impl From<BikeDetailBase> for BikeDetailPageTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images,
            logged_in: value.logged_in,
        }
    }
}

impl From<BikeDetailBase> for BikeDetailContentTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images,
            logged_in: value.logged_in,
        }
    }
}


#[derive(Template)]
#[template(path = "bike/admin/detail/page.html")]
pub struct BikeDetailAdminPageTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "bike/admin/detail/content.html")]
pub struct BikeDetailAdminContentTemplate {
    pub bike: BikeDetail,
    pub bike_images: Vec<String>,
    pub logged_in: bool,
}

impl From<BikeDetailBase> for BikeDetailAdminPageTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images,
            logged_in: value.logged_in,
        }
    }
}

impl From<BikeDetailBase> for BikeDetailAdminContentTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike,
            bike_images: value.bike_images,
            logged_in: value.logged_in,
        }
    }
}

#[derive(Template)]
#[template(path = "bike/admin/upload/content.html")]
pub struct BikeUploadFormTemplate {
    pub message: String,
}

