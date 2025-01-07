use crate::database::common::EntityById;
use crate::database::models::Id;
use serde::Serialize;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Tag {
    pub id: Id,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone, Serialize)]
pub struct TagJoin {
    pub id: Id,
    pub name: String,
    pub bike_id: Id
}

impl EntityById for Tag {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct TagCreate {
    pub name: String,
}


impl TagCreate {
    #[inline]
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TagAssign {
    pub tags_ids: Vec<Id>,
    pub bike_id: Id,
}

impl TagAssign {
    pub fn new(tags_ids: &[Id], bike_id: &Id) -> Self {
        Self {
            tags_ids: tags_ids.to_owned(),
            bike_id: *bike_id,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TagUnassign {
    pub tag_id: Id,
    pub bike_id: Id,
}

impl TagUnassign {
    #[allow(dead_code)]
    pub fn new(tag_id: &Id, bike_id: &Id) -> Self {
        Self {
            tag_id: *tag_id,
            bike_id: *bike_id,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TagSearch {
    pub id: Option<Id>,
    pub name: Option<String>,
    pub bike_id: Option<Id>,
}

impl TagSearch {
    #[must_use]
    #[inline]
    pub fn new(id: Option<&Id>, name: Option<&str>, bike_id: Option<&Id>) -> Self {
        Self {
            id: id.cloned(),
            name: name.map(|n| n.to_owned()),
            bike_id: bike_id.cloned(),
        }
    }
}
