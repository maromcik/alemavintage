use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BrandCreateForm {
    pub name: String,
    pub description: String,
}