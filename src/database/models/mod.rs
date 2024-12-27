use crate::database::common::EntityById;

pub(crate) mod bike;
pub mod brand;
pub mod model;
pub(crate) mod user;
pub mod tag;

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

pub struct AppImage {
    pub path: String,
    pub width: i32,
    pub height: i32,
}

impl AppImage {
    pub fn new(path: &str, width: i32, height: i32) -> Self {
        Self {
            path: path.to_owned(),
            width,
            height,
        }
    }
}

pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
}

impl ImageDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
