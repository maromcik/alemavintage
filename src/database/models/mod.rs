use crate::database::common::EntityById;

pub(crate) mod bike;
pub(crate) mod user;
pub mod model;
pub mod brand;

pub type Id = i64;

pub struct GetById {
    pub id: Id,
    pub fetch_deleted: bool
}

impl GetById {
    pub fn new(id: Id) -> Self {
        Self { id, fetch_deleted: false }
    }

    pub fn new_with_deleted(id: Id) -> Self {
        Self { id, fetch_deleted: true }
    }
}

impl EntityById for GetById {
    fn id(&self) -> Id {
        self.id
    }

    fn is_deleted(&self) -> bool {
        self.fetch_deleted
    }
}