use crate::database::common::EntityById;
use crate::database::models::Id;
use serde::Serialize;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Brand {
    pub id: Id,
    pub name: String,
    pub description: String,
}

impl EntityById for Brand {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

#[derive(Serialize)]
pub struct BrandDisplay {
    pub id: Id,
    pub name: String,
    pub description: String,
}

impl From<Brand> for BrandDisplay {
    fn from(value: Brand) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: markdown::to_html(&value.description),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrandCreate {
    pub name: String,
    pub description: String,
}

impl BrandCreate {
    #[inline]
    #[allow(dead_code)]
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BrandSearch {
    pub name: Option<String>,
}

impl BrandSearch {
    #[must_use]
    #[inline]
    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrandUpdate {
    pub id: Id,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl BrandUpdate {
    #[allow(dead_code)]
    pub fn new(id: &Id, name: Option<&str>, description: Option<&str>) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
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
