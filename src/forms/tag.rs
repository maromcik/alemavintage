use serde::Deserialize;
use crate::database::models::Id;

#[derive(Debug, Clone, Deserialize)]
pub struct TagsAssignForm {
    pub bike_id: Id,
    pub tags: Vec<Id>,
}
