use serde::Deserialize;
use crate::database::models::Id;

#[derive(Debug, Clone, Deserialize)]
pub struct BrandCreateForm {
    pub name: String,
    pub description: String,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BrandEditForm {
    pub id: Id,
    pub name: String,
    pub description: String,
}