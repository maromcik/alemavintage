use askama::Template;
#[derive(Template)]
#[template(path = "error.html")]
pub struct GenericError {
    pub code: String,
    pub description: String,
}
