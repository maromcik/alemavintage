use askama::Template;
use crate::database::models::bike::{Bike, BikeDetail};

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
