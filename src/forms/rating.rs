use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct RatingCreateForm {
    pub rating: i16,
    pub review: Option<String>,
}
