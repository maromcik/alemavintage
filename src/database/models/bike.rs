use crate::database::common::query_parameters::{
    DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable,
};
use crate::database::common::EntityById;
use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Bike {
    pub id: Id,
    // --------------
    pub name: String,
    pub model_id: Id,
    pub view_count: i64,
    pub like_count: i64,
    pub description: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl EntityById for Bike {
    fn id(&self) -> Id {
        self.id
    }
    fn fetch_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
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
    pub model_name: String,
}

impl EntityById for BikeDetail {
    fn id(&self) -> Id {
        self.id
    }
    fn fetch_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }
}

#[derive(Serialize)]
pub struct BikeDisplay {
    pub id: Id,
    // --------------
    pub name: String,
    pub brand_id: Id,
    pub model_id: Id,
    pub view_count: i64,
    pub like_count: i64,
    pub thumbnail: String,
    pub description: String,
    pub deleted: bool,

    pub brand_name: String,
    pub model_name: String,
}

impl From<BikeDetail> for BikeDisplay {
    fn from(value: BikeDetail) -> Self {
        Self {
            id: value.id,
            name: value.name,
            brand_id: value.brand_id,
            model_id: value.model_id,
            view_count: value.view_count,
            like_count: value.like_count,
            thumbnail: value.thumbnail,
            description: markdown::to_html(&value.description),
            deleted: value.deleted_at.is_some(),
            brand_name: value.brand_name,
            model_name: value.model_name,
        }
    }
}

pub struct BikeCreate {
    pub name: String,
    pub model_id: Id,
    pub thumbnail: String,
    pub description: String,
}

impl BikeCreate {
    pub fn new(name: &str, model_id: Id, thumbnail: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            model_id,
            thumbnail: thumbnail.to_owned(),
            description: description.to_owned(),
        }
    }
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

    pub fn search_by_model_id(model_id: Id, query_params: DbQueryParams) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: Some(model_id),
            query_params,
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
    pub fn search_by_bike_name(name: &str) -> Self {
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

#[derive(Debug, Clone)]
pub struct BikeMetadataForm {
    pub name: String,
    pub description: String,
    pub model_id: Id,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct BikeImage {
    pub id: Id,
    pub bike_id: Id,
    pub path: String,
}

impl EntityById for BikeImage {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_deleted(&self) -> bool {
        false
    }
}

pub struct BikeImageSearch {
    pub bike_id: Option<Id>,
    pub query_params: DbQueryParams,
}

impl BikeImageSearch {
    pub fn new(bike_id: Option<Id>) -> Self {
        Self {
            bike_id,
            query_params: DbQueryParams::default(),
        }
    }

    #[allow(dead_code)]
    pub fn with_params(bike_id: Option<Id>, query_params: DbQueryParams) -> Self {
        Self {
            bike_id,
            query_params,
        }
    }

    #[allow(dead_code)]
    pub fn search_by_bike_id(bike_id: Id) -> Self {
        Self {
            bike_id: Some(bike_id),
            query_params: DbQueryParams::order(
                DbOrderColumn::new(DbTable::BikeImage, DbColumn::Path, DbOrder::Asc),
                None,
            ),
        }
    }
}

pub struct BikeImageCreate {
    pub bike_id: Id,
    pub paths: Vec<String>,
}

impl BikeImageCreate {
    pub fn new(bike_id: Id, paths: Vec<String>) -> Self {
        Self { bike_id, paths }
    }
}

pub struct BikeUpdate {
    pub id: Id,
    pub model_id: Option<Id>,
    pub name: Option<String>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
    pub view_count: Option<i64>,
    pub like_count: Option<i64>,
}

impl BikeUpdate {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Id,
        name: Option<&str>,
        model_id: Option<&Id>,
        thumbnail: Option<&str>,
        description: Option<&str>,
        view_count: Option<&i64>,
        like_count: Option<&i64>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            name: name.and_then(change_to_owned),
            model_id: model_id.copied(),
            thumbnail: thumbnail.and_then(change_to_owned),
            description: description.and_then(change_to_owned),
            view_count: view_count.copied(),
            like_count: like_count.copied(),
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none()
            && self.model_id.is_none()
            && self.view_count.is_none()
            && self.like_count.is_none()
            && self.description.is_none()
            && self.thumbnail.is_none()
    }

    #[allow(dead_code)]
    pub fn update_views(id: Id, view_count: i64) -> Self {
        Self {
            id,
            model_id: None,
            name: None,
            thumbnail: None,
            description: None,
            view_count: Some(view_count),
            like_count: None,
        }
    }
}

pub struct BikeGetById {
    pub id: Id,
    pub fetch_deleted: bool,
    pub update_view_count: bool,
}

impl EntityById for BikeGetById {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_deleted(&self) -> bool {
        self.fetch_deleted
    }
}

impl BikeGetById {
    pub fn new(id: Id, fetch_deleted: bool, update_view_count: bool) -> Self {
        Self {
            id,
            fetch_deleted,
            update_view_count,
        }
    }

    pub fn new_admin(id: Id) -> Self {
        Self {
            id,
            fetch_deleted: true,
            update_view_count: false,
        }
    }
}

pub struct BikeCreateSessionKeys {
    pub name: String,
    pub description: String,
    pub model_id: String,
}

impl BikeCreateSessionKeys {
    pub fn new(user_id: Id) -> Self {
        Self {
            name: format!("bike_create_{user_id}_name"),
            description: format!("bike_create_{user_id}_description"),
            model_id: format!("bike_create_{user_id}_model_id"),
        }
    }
}

pub struct BikeDetailSessionKeys {
    pub visited: String,
}

impl BikeDetailSessionKeys {
    pub fn new(bike_id: Id) -> Self {
        Self {
            visited: format!("bike_{bike_id}_visited"),
        }
    }
}
