use serde::Serialize;
use crate::database::models::tag::{TagJoin};

#[derive(Serialize)]
pub struct TagsTemplate {
    pub tags: Vec<TagJoin>,
}