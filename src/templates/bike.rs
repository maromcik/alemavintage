// use askama::Template;
// use crate::database::models::bike::Bike;
// 
// #[derive(Template)]
// #[template(path = "bike.html")]
// pub struct NewReleasesPageTemplate {
//     pub bikes: Vec<Bike>,
// }

use askama::Template;
use crate::database::models::bike::BikeDetail;

#[derive(Template)]
#[template(path = "studio_create_bike.html")]
pub struct BikeCreatePageTemplate {
    // pub genres: Vec<Genre>,
}

#[derive(Template)]
#[template(path = "bike/bike_create.html")]
pub struct BikeCreateContentTemplate {
    // pub genres: Vec<Genre>,
}

#[derive(Template)]
#[template(path = "detail.html")]
pub struct BikeDetailPageTemplate {
    pub bike: BikeDetail
}

#[derive(Template)]
#[template(path = "bike/detail-content.html")]
pub struct BikeDetailContentTemplate {
    pub bike: BikeDetail
}

pub struct BikeDetailBase {
    pub bike: BikeDetail
}

impl From<BikeDetailBase> for BikeDetailPageTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike
        }
    }
}

impl From<BikeDetailBase> for BikeDetailContentTemplate {
    fn from(value: BikeDetailBase) -> Self {
        Self {
            bike: value.bike
        }
    }
}

#[derive(Template)]
#[template(path = "bike/bike_upload.html")]
pub struct BikeUploadFormTemplate {
    pub message: String,
}