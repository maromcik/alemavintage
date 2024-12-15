use askama::Template;
use crate::database::models::bike::{Bike, BikeDetail};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub email: String,
    pub bikes: Vec<BikeDetail>,
    pub thumbnail: String,
}

#[derive(Template)]
#[template(path = "index_content.html")]
pub struct IndexContentTemplate {
    pub logged_in: bool,
    pub email: String,
    pub bikes: Vec<BikeDetail>,
    pub thumbnail: String,
}

pub struct IndexBase {
    pub logged_in: bool,
    pub email: String,
    pub bikes: Vec<BikeDetail>,
    pub thumbnail: String,
}

impl From<IndexBase> for IndexContentTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
            email: value.email,
            bikes: value.bikes,
            thumbnail: value.thumbnail,
        }
    }
}

impl From<IndexBase> for IndexTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
            email: value.email,
            bikes: value.bikes,
            thumbnail: value.thumbnail,
        }
    }
}
