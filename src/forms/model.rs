use serde::Deserialize;
use crate::database::models::Id;

#[derive(Debug, Clone, Deserialize)]
pub struct ModelCreateForm {
    pub brand_id: Id,
    pub name: String,
    pub description: String,
}