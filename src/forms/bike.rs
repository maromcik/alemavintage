use crate::database::models::Id;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BikeCreateForm {
    pub name: String,
    pub description: String,
    pub model_id: Id,
}
#[derive(Debug, MultipartForm)]
pub struct BikeUploadForm {
    #[multipart(rename = "thumbnail")]
    pub thumbnail: TempFile,
    #[multipart(rename = "files")]
    pub photos: Vec<TempFile>,
}

#[derive(Debug, MultipartForm)]
pub struct BikeThumbnailEditForm {
    #[multipart(rename = "thumbnail")]
    pub thumbnail: TempFile,
    pub bike_id: Text<Id>,
}

#[derive(Debug, MultipartForm)]
pub struct BikeImagesEditForm {
    #[multipart(rename = "files")]
    pub photos: Vec<TempFile>,
    pub bike_id: Text<Id>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BikeEditForm {
    pub bike_id: Id,
    pub name: String,
    pub model_id: Id,
    pub description: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct BikeQuickSearchQuery {
    pub query: String,
    pub search_type: String,
}
