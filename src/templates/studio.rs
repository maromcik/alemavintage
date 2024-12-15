use askama::Template;
use crate::database::models::bike::{Bike, BikeDetail};

pub struct StudioBase {
    pub bikes: Vec<BikeDetail>,
}

impl From<StudioBase> for StudioPageTemplate {
    fn from(value: StudioBase) -> Self {
        Self {
            bikes: value.bikes,
        }
    }
}

impl From<StudioBase> for StudioContentTemplate {
    fn from(value: StudioBase) -> Self {
        Self {
            bikes: value.bikes,
        }
    }
}

#[derive(Template)]
#[template(path = "studio.html")]
pub struct StudioPageTemplate {
    pub bikes: Vec<BikeDetail>,
}

#[derive(Template)]
#[template(path = "bike/studio-content.html")]
pub struct StudioContentTemplate {
    pub bikes: Vec<BikeDetail>,
}
