use crate::database::common::EntityById;
use crate::database::models::brand::Brand;
use crate::database::models::Id;


#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Model {
    pub id: Id,
    pub name: String,
    pub description: String,
}

impl EntityById for Model {
    fn id(&self) -> Id {
        self.id
    }

    fn is_deleted(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct ModelCreate {
    pub name: String,
    pub description: String,
}

impl ModelCreate {
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
pub struct ModelSearch {
    pub name: Option<String>,
}

impl ModelSearch {
    #[must_use]
    #[inline]
    pub fn new(name: Option<&str>) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelUpdate {
    pub id: Id,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl ModelUpdate {
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