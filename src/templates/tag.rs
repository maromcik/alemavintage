use serde::Serialize;
use crate::database::models::tag::{TagJoin};

#[derive(Serialize)]
pub struct TagsTemplate<'a> {
    pub tags: &'a Vec<TagJoin>,
}