use serde::Serialize;

#[derive(Serialize)]
pub struct IndexTemplate {
    pub logged_in: bool,
}

#[derive(Serialize)]
pub struct AboutTemplate {
    pub logged_in: bool,
}
