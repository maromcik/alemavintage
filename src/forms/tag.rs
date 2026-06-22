use crate::database::models::Id;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct TagsAssignForm {
    pub bike_id: Id,
    pub tags: Vec<Id>,
}
