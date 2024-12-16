use crate::database::models::bike::BikeDetail;
use serde::Serialize;


#[derive(Serialize)]
pub struct StudioTemplate {
    pub bikes: Vec<BikeDetail>,
    pub logged_in: bool,
}

