use crate::database::common::EntityById;
use crate::database::models::Id;
use serde::Serialize;
use crate::database::common::query_parameters::DbQueryParams;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Model {
    pub id: Id,
    pub brand_id: Id,
    pub name: String,
    pub description: String,
}

impl EntityById for Model {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone, Serialize)]
pub struct ModelDetail {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub brand_id: Id,
    pub brand_name: String,
    pub brand_description: String,
}

impl EntityById for ModelDetail {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

#[derive(Serialize)]
pub struct ModelDisplay {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub brand_id: Id,
    pub brand_name: String,
    pub brand_description: String,
}

impl From<ModelDetail> for ModelDisplay {
    fn from(value: ModelDetail) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: markdown::to_html(&value.description),
            brand_id: value.brand_id,
            brand_name: value.brand_name,
            brand_description: markdown::to_html(&value.brand_description),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelCreate {
    pub name: String,
    pub brand_id: Id,
    pub description: String,
}

impl ModelCreate {
    #[inline]
    #[allow(dead_code)]
    pub fn new(brand_id: &Id, name: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            brand_id: *brand_id,
            description: description.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ModelSearch {
    pub name: Option<String>,
    pub brand_id: Option<Id>,
    pub query_params: DbQueryParams,
}

impl ModelSearch {
    #[must_use]
    #[inline]
    pub fn new(brand_id: Option<&Id>, name: Option<&str>, query_params: DbQueryParams) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
            brand_id: brand_id.copied(),
            query_params
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelUpdate {
    pub id: Id,
    pub brand_id: Option<Id>,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl ModelUpdate {
    #[allow(dead_code)]
    pub fn new(
        id: &Id,
        brand_id: Option<&Id>,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            brand_id: brand_id.copied(),
            name: name.and_then(change_to_owned),
            description: description.and_then(change_to_owned),
        }
    }

    #[inline]
    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none() && self.description.is_none()
    }
}
