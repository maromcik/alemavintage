use crate::database::models::tag::TagJoin;
use serde::Serialize;

#[derive(Serialize)]
pub struct TagsTemplate<'a> {
    pub tags: &'a Vec<TagJoin>,
}
