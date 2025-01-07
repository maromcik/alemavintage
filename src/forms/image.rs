use crate::database::models::Id;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;


#[derive(Debug, MultipartForm)]
pub struct ImageUploadForm {
    pub image_type: Text<Id>,
    #[multipart(rename = "files")]
    pub photos: Vec<TempFile>,
}