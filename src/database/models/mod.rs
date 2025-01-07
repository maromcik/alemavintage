use crate::database::common::EntityById;

pub mod bike;
pub mod brand;
pub mod model;
pub mod user;
pub mod tag;
pub mod image;

pub type Id = i64;

pub struct GetById {
    pub id: Id,
    pub fetch_deleted: bool,
}

impl GetById {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            fetch_deleted: false,
        }
    }

    pub fn new_with_deleted(id: Id) -> Self {
        Self {
            id,
            fetch_deleted: true,
        }
    }
}

impl EntityById for GetById {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        self.fetch_deleted
    }
}
