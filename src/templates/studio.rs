use askama::Template;
use crate::database::models::bike::{Bike, BikeDetail};

pub struct StudioBase {
    pub bikes: Vec<BikeDetail>,
    pub logged_in: bool,
}

impl From<StudioBase> for StudioPageTemplate {
    fn from(value: StudioBase) -> Self {
        Self {
            bikes: value.bikes,
            logged_in: value.logged_in,
        }
    }
}

impl From<StudioBase> for StudioContentTemplate {
    fn from(value: StudioBase) -> Self {
        Self {
            bikes: value.bikes,
            logged_in: value.logged_in,
        }
    }
}

#[derive(Template)]
#[template(path = "studio/page.html")]
pub struct StudioPageTemplate {
    pub bikes: Vec<BikeDetail>,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "studio/content.html")]
pub struct StudioContentTemplate {
    pub bikes: Vec<BikeDetail>,
    pub logged_in: bool,
}
