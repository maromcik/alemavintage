use serde::Serialize;
use crate::database::models::bike::BikeDisplay;

#[derive(Serialize)]
pub struct IndexTemplate {
    pub logged_in: bool,
    pub bikes: Vec<BikeDisplay>
}

#[derive(Serialize)]
pub struct AboutTemplate {
    pub logged_in: bool,
}
