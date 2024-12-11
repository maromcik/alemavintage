use crate::database::common::{EntityById};
use crate::database::models::Id;
use chrono::{DateTime, Utc};
use crate::database::common::query_parameters::DbQueryParams;

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

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct BikeDetail {
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
    
    pub brand_name: String,
    pub model_name: String
}

impl EntityById for BikeDetail {
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


#[derive(Debug, Clone)]
pub struct BikeSearch {
    pub name: Option<String>,
    pub brand_name: Option<String>,
    pub model_name: Option<String>,
    pub brand_id: Option<Id>,
    pub model_id: Option<Id>,
    pub query_params: DbQueryParams,
}


impl BikeSearch {
    #[inline]
    #[allow(dead_code)]
    pub fn new(
        name: Option<&str>,
        brand_id: Option<Id>,
        brand_name: Option<&str>,
        model_id: Option<Id>,
        model_name: Option<&str>,
        query_params: DbQueryParams,
    ) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
            model_name: model_name.map(|n| n.to_owned()),
            brand_name: brand_name.map(|n| n.to_owned()),
            brand_id: brand_id.map(|n| n.to_owned()),
            model_id: model_id.map(|n| n.to_owned()),
            query_params,
        }
    }
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            name: None,
            brand_name: None,
            model_name: None,
            brand_id: None,
            model_id: None,
            query_params: Default::default(),
        }
    }
    pub fn with_params(query_params: DbQueryParams) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: None,
            query_params,
        }
    }

    pub fn search_by_model_id(model_id: Id) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: Some(model_id),
            query_params: DbQueryParams::default(),
        }
    }

    pub fn search_by_brand_id(brand_id: Id, query_params: DbQueryParams) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: Some(brand_id),
            model_id: None,
            query_params,
        }
    }

    #[allow(dead_code)]
    pub fn search_by_book_name(name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: None,
            query_params: DbQueryParams::default(),
        }
    }
    #[allow(dead_code)]
    pub fn search_by_model_name(name: &str) -> Self {
        Self {
            name: None,
            model_name: Some(name.to_owned()),
            brand_name: None,
            brand_id: None,
            model_id: None,
            query_params: DbQueryParams::default(),
        }
    }
    #[allow(dead_code)]
    pub fn search_by_brand_name(name: &str) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: Some(name.to_owned()),
            brand_id: None,
            model_id: None,
            query_params: DbQueryParams::default(),
        }
    }
}