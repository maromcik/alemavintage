use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "index_content.html")]
pub struct IndexContentTemplate {
    pub logged_in: bool,
}

pub struct IndexBase {
    pub logged_in: bool,
}

impl From<IndexBase> for IndexContentTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
        }
    }
}

impl From<IndexBase> for IndexTemplate {
    fn from(value: IndexBase) -> Self {
        Self {
            logged_in: value.logged_in,
        }
    }
}
