use crate::database::models::Id;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AudiobookCreateForm {
    pub name: String,
    pub description: String,
    pub genre_id: Id,
}
#[derive(Debug, MultipartForm)]
pub struct AudiobookUploadForm {
    #[multipart(rename = "thumbnail")]
    pub thumbnail: Option<TempFile>,
    #[multipart(rename = "file")]
    pub audio_file: TempFile,
}

#[derive(Debug, MultipartForm)]
pub struct AudiobookThumbnailEditForm {
    #[multipart(rename = "thumbnail")]
    pub thumbnail: TempFile,
    pub audiobook_id: Text<Id>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AudiobookEditForm {
    pub audiobook_id: Id,
    pub name: String,
    pub genre_id: Id,
    pub description: String,
}

#[derive(Deserialize)]
pub struct AudiobookQuickSearchQuery {
    pub query: String,
    pub search_type: String,
}
