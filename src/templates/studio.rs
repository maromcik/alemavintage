use crate::database::models::bike::BikeDisplay;
use serde::Serialize;

#[derive(Serialize)]
pub struct StudioTemplate {
    pub bikes: Vec<BikeDisplay>,
    pub logged_in: bool,
}
