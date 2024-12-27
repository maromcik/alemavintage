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
    pub year: i32,
    pub price: f64,
    pub height: i32,
    pub top_tube_size: i32,
    pub frame: String,
    pub seat_tube_sizes: String,
    pub headset: String,
    pub crankset: String,
    pub bottom_bracket: String,
    pub front_derail: String,
    pub rear_derail: String,
    pub brakes: String,
    pub shifters: String,
    pub brake_levers: String,
    pub saddle: String,
    pub seat_post: String,
    pub hubs: String,
    pub rims: String,
    pub handlebar: String,
    pub stem: String,
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
    pub delete_existing: Option<Text<bool>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BikeEditForm {
    pub bike_id: Id,
    pub name: String,
    pub model_id: Id,
    pub description: String,
    pub year: i32,
    pub price: f64,
    pub height: i32,
    pub top_tube_size: i32,
    pub frame: String,
    pub seat_tube_sizes: String,
    pub headset: String,
    pub crankset: String,
    pub bottom_bracket: String,
    pub front_derail: String,
    pub rear_derail: String,
    pub brakes: String,
    pub shifters: String,
    pub brake_levers: String,
    pub saddle: String,
    pub seat_post: String,
    pub hubs: String,
    pub rims: String,
    pub handlebar: String,
    pub stem: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct BikeQuickSearchQuery {
    pub query: String,
    pub search_type: String,
}
