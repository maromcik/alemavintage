use crate::database::common::{EntityById};
use crate::database::models::Id;
use chrono::{DateTime, Utc};


#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Bike {
    pub id: Id,
    // --------------
    pub name: String,
    pub brand_id: Id,
    pub model_id: Id,
    pub view_count: i64,
    pub like_count: i64,
    pub thumbnail: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl EntityById for Bike {
    fn id(&self) -> Id {
        self.id
    }
    fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

}

pub struct BikeCreate {
    pub name: String,
    pub brand_id: Id,
    pub model_id: Id,
    pub view_count: i64,
    pub like_count: i64,
    pub thumbnail: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}