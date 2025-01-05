use crate::database::common::query_parameters::{
    DbColumn, DbOrder, DbOrderColumn, DbQueryParams, DbTable,
};
use crate::database::common::EntityById;
use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Bike {
    pub id: Id,
    // --------------
    pub name: String,
    pub model_id: Id,
    pub view_count: i64,
    pub like_count: i64,
    pub description: String,
    pub preview: Option<Id>,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub hidden: bool,
    pub year: i32,
    pub price: i32,
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
    pub status: Option<String>,
}

impl EntityById for Bike {
    fn id(&self) -> Id {
        self.id
    }
    fn fetch_hidden(&self) -> bool {
        self.hidden
    }
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct BikeDetail {
    pub id: Id,
    // --------------
    pub model_id: Id,
    pub name: String,
    pub description: String,
    pub view_count: i64,
    pub like_count: i64,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub hidden: bool,

    pub year: i32,
    pub price: i32,
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
    pub status: Option<String>,

    pub brand_id: Id,
    pub brand_name: String,
    pub model_name: String,

    pub preview_path: Option<String>,
    pub preview_width: Option<i32>,
    pub preview_height: Option<i32>,
    pub preview_thumbnail_path: Option<String>,
}

impl EntityById for BikeDetail {
    fn id(&self) -> Id {
        self.id
    }
    fn fetch_hidden(&self) -> bool {
        self.hidden
    }
}

#[derive(Serialize)]
pub struct BikeDisplay {
    pub id: Id,
    // --------------
    pub name: String,
    pub brand_id: Id,
    pub model_id: Id,
    pub view_count: i64,
    pub like_count: i64,
    pub description: String,
    pub hidden: bool,

    pub year: i32,
    pub price: String,
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
    pub status: String,

    pub brand_name: String,
    pub model_name: String,

    pub preview_path: String,
    pub preview_width: i32,
    pub preview_height: i32,
    pub preview_thumbnail_path: String,
}

impl From<BikeDetail> for BikeDisplay {
    fn from(value: BikeDetail) -> Self {
        Self {
            id: value.id,
            name: value.name,
            brand_id: value.brand_id,
            model_id: value.model_id,
            view_count: value.view_count,
            like_count: value.like_count,
            description: value.description,
            hidden: value.hidden,
            year: value.year,
            price: format!("{:.2}", value.price as f64 / 100_f64),
            height: value.height,
            top_tube_size: value.top_tube_size,
            frame: value.frame,
            seat_tube_sizes: value.seat_tube_sizes,
            headset: value.headset,
            crankset: value.crankset,
            bottom_bracket: value.bottom_bracket,
            front_derail: value.front_derail,
            rear_derail: value.rear_derail,
            brakes: value.brakes,
            shifters: value.shifters,
            brake_levers: value.brake_levers,
            saddle: value.saddle,
            seat_post: value.seat_post,
            hubs: value.hubs,
            rims: value.rims,
            handlebar: value.handlebar,
            stem: value.stem,
            status: value
                .status
                .unwrap_or("<p>NO IMAGES FOUND</p>".to_string()),
            brand_name: value.brand_name,
            model_name: value.model_name,
            preview_path: value.preview_path.unwrap_or("/static/images/logo.png".to_string()),
            preview_width: value.preview_width.unwrap_or(400),
            preview_height: value.preview_height.unwrap_or(400),
            preview_thumbnail_path: value.preview_thumbnail_path.unwrap_or("/static/images/logo.png".to_string()),
        }
    }
}

impl BikeDisplay {
    pub fn description_to_markdown(mut self) -> BikeDisplay {
        self.description = markdown::to_html(&self.description);
        self
    }
}

pub struct BikeCreate {
    pub name: String,
    pub model_id: Id,
    pub preview: Option<Id>,
    pub description: String,

    pub year: i32,
    pub price: i32,
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

impl BikeCreate {
    pub fn new(
        name: &str,
        model_id: Id,
        preview: Option<Id>,
        description: &str,
        year: &i32,
        price: &i32,
        height: &i32,
        top_tube_size: &i32,
        frame: &str,
        seat_tube_sizes: &str,
        headset: &str,
        crankset: &str,
        bottom_bracket: &str,
        front_derail: &str,
        rear_derail: &str,
        brakes: &str,
        shifters: &str,
        brake_levers: &str,
        saddle: &str,
        seat_post: &str,
        hubs: &str,
        rims: &str,
        handlebar: &str,
        stem: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            model_id,
            preview,
            description: description.to_owned(),
            year: *year,
            price: *price,
            height: *height,
            top_tube_size: *top_tube_size,
            frame: frame.to_owned(),
            seat_tube_sizes: seat_tube_sizes.to_owned(),
            headset: headset.to_owned(),
            crankset: crankset.to_owned(),
            bottom_bracket: bottom_bracket.to_owned(),
            front_derail: front_derail.to_owned(),
            rear_derail: rear_derail.to_owned(),
            brakes: brakes.to_owned(),
            shifters: shifters.to_owned(),
            brake_levers: brake_levers.to_owned(),
            saddle: saddle.to_owned(),
            seat_post: seat_post.to_owned(),
            hubs: hubs.to_owned(),
            rims: rims.to_owned(),
            handlebar: handlebar.to_owned(),
            stem: stem.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BikeSearch {
    pub name: Option<String>,
    pub brand_name: Option<String>,
    pub model_name: Option<String>,
    pub brand_id: Option<Id>,
    pub model_id: Option<Id>,
    pub tag_id: Option<Id>,
    pub query_params: DbQueryParams,
}

impl BikeSearch {
    #[inline]
    #[allow(dead_code)]
    pub fn new(
        name: Option<&str>,
        brand_id: Option<Id>,
        brand_name: Option<&str>,
        model_id: Option<Id>,
        model_name: Option<&str>,
        tag_id: Option<Id>,
        query_params: DbQueryParams,
    ) -> Self {
        Self {
            name: name.map(|n| n.to_owned()),
            model_name: model_name.map(|n| n.to_owned()),
            brand_name: brand_name.map(|n| n.to_owned()),
            brand_id: brand_id.map(|n| n.to_owned()),
            model_id: model_id.map(|n| n.to_owned()),
            tag_id: tag_id.map(|n| n.to_owned()),
            query_params,
        }
    }
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            name: None,
            brand_name: None,
            model_name: None,
            brand_id: None,
            model_id: None,
            tag_id: None,
            query_params: Default::default(),
        }
    }
    pub fn with_params(query_params: DbQueryParams) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: None,
            tag_id: None,
            query_params,
        }
    }

    pub fn search_by_model_id(model_id: Id, query_params: DbQueryParams) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: Some(model_id),
            tag_id: None,
            query_params,
        }
    }

    pub fn search_by_brand_id(brand_id: Id, query_params: DbQueryParams) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: Some(brand_id),
            model_id: None,
            tag_id: None,
            query_params,
        }
    }

    #[allow(dead_code)]
    pub fn search_by_bike_name(name: &str) -> Self {
        Self {
            name: Some(name.to_owned()),
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: None,
            tag_id: None,
            query_params: DbQueryParams::default(),
        }
    }
    #[allow(dead_code)]
    pub fn search_by_model_name(name: &str) -> Self {
        Self {
            name: None,
            model_name: Some(name.to_owned()),
            brand_name: None,
            brand_id: None,
            model_id: None,
            tag_id: None,
            query_params: DbQueryParams::default(),
        }
    }
    #[allow(dead_code)]
    pub fn search_by_brand_name(name: &str) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: Some(name.to_owned()),
            brand_id: None,
            model_id: None,
            tag_id: None,
            query_params: DbQueryParams::default(),
        }
    }

    #[allow(dead_code)]
    pub fn search_by_tag_id(tag_id: Id) -> Self {
        Self {
            name: None,
            model_name: None,
            brand_name: None,
            brand_id: None,
            model_id: None,
            tag_id: Some(tag_id),
            query_params: DbQueryParams::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BikeMetadataForm {
    pub bike_id: Id,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Serialize)]
pub struct BikeImage {
    pub id: Id,
    pub bike_id: Option<Id>,
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub thumbnail_path: String,
}

impl EntityById for BikeImage {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

pub struct BikeImageSearch {
    pub bike_id: Option<Id>,
    pub query_params: DbQueryParams,
}

impl BikeImageSearch {
    pub fn new(bike_id: Option<Id>) -> Self {
        Self {
            bike_id,
            query_params: DbQueryParams::order(
                DbOrderColumn::new_column_only(DbColumn::Id, DbOrder::Asc),
                None,
            ),
        }
    }

    #[allow(dead_code)]
    pub fn with_params(bike_id: Option<Id>, query_params: DbQueryParams) -> Self {
        Self {
            bike_id,
            query_params,
        }
    }

    #[allow(dead_code)]
    pub fn search_by_bike_id(bike_id: Id) -> Self {
        Self {
            bike_id: Some(bike_id),
            query_params: DbQueryParams::order(
                DbOrderColumn::new(DbTable::BikeImage, DbColumn::Path, DbOrder::Asc),
                None,
            ),
        }
    }
}

pub struct BikeImageCreate {
    pub path: String,
    pub width: i32,
    pub height: i32,
    pub thumbnail_path: String,
}

impl BikeImageCreate {
    pub fn new(path: &str, width: &i32, height: &i32, thumbnail_path: &String) -> Self {
        Self {
            path: path.to_owned(),
            width: *width,
            height: *height,
            thumbnail_path: thumbnail_path.to_owned(),
        }
    }
}

pub struct BikeImagesCreate {
    pub bike_id: Option<Id>,
    pub bike_images: Vec<BikeImageCreate>,
}

impl BikeImagesCreate {
    pub fn new(bike_id: Id, paths: Vec<BikeImageCreate>) -> Self {
        Self {
            bike_id: Some(bike_id),
            bike_images: paths,
        }
    }
}

pub struct BikeUpdate {
    pub id: Id,
    pub model_id: Option<Id>,
    pub name: Option<String>,
    pub preview: Option<Id>,
    pub description: Option<String>,
    pub view_count: Option<i64>,
    pub like_count: Option<i64>,
    pub hidden: Option<bool>,

    pub year: Option<i32>,
    pub price: Option<i32>,
    pub height: Option<i32>,
    pub top_tube_size: Option<i32>,
    pub frame: Option<String>,
    pub seat_tube_sizes: Option<String>,
    pub headset: Option<String>,
    pub crankset: Option<String>,
    pub bottom_bracket: Option<String>,
    pub front_derail: Option<String>,
    pub rear_derail: Option<String>,
    pub brakes: Option<String>,
    pub shifters: Option<String>,
    pub brake_levers: Option<String>,
    pub saddle: Option<String>,
    pub seat_post: Option<String>,
    pub hubs: Option<String>,
    pub rims: Option<String>,
    pub handlebar: Option<String>,
    pub stem: Option<String>,
    pub status: Option<String>,
}

impl BikeUpdate {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Id,
        name: Option<&str>,
        model_id: Option<&Id>,
        preview: Option<&Id>,
        description: Option<&str>,
        view_count: Option<&i64>,
        like_count: Option<&i64>,
        hidden: Option<&bool>,
        year: Option<&i32>,
        price: Option<&i32>,
        height: Option<&i32>,
        top_tube_size: Option<&i32>,
        frame: Option<&str>,
        seat_tube_sizes: Option<&str>,
        headset: Option<&str>,
        crankset: Option<&str>,
        bottom_bracket: Option<&str>,
        front_derail: Option<&str>,
        rear_derail: Option<&str>,
        brakes: Option<&str>,
        shifters: Option<&str>,
        brake_levers: Option<&str>,
        saddle: Option<&str>,
        seat_post: Option<&str>,
        hubs: Option<&str>,
        rims: Option<&str>,
        handlebar: Option<&str>,
        stem: Option<&str>,
        status: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            name: name.and_then(change_to_owned),
            model_id: model_id.copied(),
            preview: preview.copied(),
            description: description.and_then(change_to_owned),
            view_count: view_count.copied(),
            like_count: like_count.copied(),
            hidden: hidden.copied(),
            year: year.copied(),
            price: price.copied(),
            height: height.copied(),
            top_tube_size: top_tube_size.copied(),
            frame: frame.and_then(change_to_owned),
            seat_tube_sizes: seat_tube_sizes.and_then(change_to_owned),
            headset: headset.and_then(change_to_owned),
            crankset: crankset.and_then(change_to_owned),
            bottom_bracket: bottom_bracket.and_then(change_to_owned),
            front_derail: front_derail.and_then(change_to_owned),
            rear_derail: rear_derail.and_then(change_to_owned),
            brakes: brakes.and_then(change_to_owned),
            shifters: shifters.and_then(change_to_owned),
            brake_levers: brake_levers.and_then(change_to_owned),
            saddle: saddle.and_then(change_to_owned),
            seat_post: seat_post.and_then(change_to_owned),
            hubs: hubs.and_then(change_to_owned),
            rims: rims.and_then(change_to_owned),
            handlebar: handlebar.and_then(change_to_owned),
            stem: stem.and_then(change_to_owned),
            status: status.and_then(change_to_owned),
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none()
            && self.model_id.is_none()
            && self.preview.is_none()
            && self.view_count.is_none()
            && self.like_count.is_none()
            && self.description.is_none()
            && self.hidden.is_none()
            && self.year.is_none()
            && self.price.is_none()
            && self.height.is_none()
            && self.top_tube_size.is_none()
            && self.frame.is_none()
            && self.seat_tube_sizes.is_none()
            && self.headset.is_none()
            && self.crankset.is_none()
            && self.bottom_bracket.is_none()
            && self.front_derail.is_none()
            && self.rear_derail.is_none()
            && self.brakes.is_none()
            && self.shifters.is_none()
            && self.brake_levers.is_none()
            && self.saddle.is_none()
            && self.seat_post.is_none()
            && self.hubs.is_none()
            && self.rims.is_none()
            && self.handlebar.is_none()
            && self.stem.is_none()
            && self.status.is_none()
    }

    #[allow(dead_code)]
    pub fn update_views(id: Id, view_count: i64) -> Self {
        Self {
            id,
            model_id: None,
            name: None,
            preview: None,
            description: None,
            view_count: Some(view_count),
            like_count: None,
            hidden: None,
            year: None,
            price: None,
            height: None,
            top_tube_size: None,
            frame: None,
            seat_tube_sizes: None,
            headset: None,
            crankset: None,
            bottom_bracket: None,
            front_derail: None,
            rear_derail: None,
            brakes: None,
            shifters: None,
            brake_levers: None,
            saddle: None,
            seat_post: None,
            hubs: None,
            rims: None,
            handlebar: None,
            stem: None,
            status: None,
        }
    }

    pub fn update_thumbnail(id: Id, preview: Id) -> Self {
        Self {
            id,
            model_id: None,
            name: None,
            preview: Some(preview),
            description: None,
            view_count: None,
            like_count: None,
            hidden: None,
            year: None,
            price: None,
            height: None,
            top_tube_size: None,
            frame: None,
            seat_tube_sizes: None,
            headset: None,
            crankset: None,
            bottom_bracket: None,
            front_derail: None,
            rear_derail: None,
            brakes: None,
            shifters: None,
            brake_levers: None,
            saddle: None,
            seat_post: None,
            hubs: None,
            rims: None,
            handlebar: None,
            stem: None,
            status: None,
        }
    }

    pub fn update_thumbnail_and_mark_complete(id: Id, preview: Id) -> Self {
        Self {
            id,
            model_id: None,
            name: None,
            preview: Some(preview),
            description: None,
            view_count: None,
            like_count: None,
            hidden: Some(false),
            year: None,
            price: None,
            height: None,
            top_tube_size: None,
            frame: None,
            seat_tube_sizes: None,
            headset: None,
            crankset: None,
            bottom_bracket: None,
            front_derail: None,
            rear_derail: None,
            brakes: None,
            shifters: None,
            brake_levers: None,
            saddle: None,
            seat_post: None,
            hubs: None,
            rims: None,
            handlebar: None,
            stem: None,
            status: None,
        }
    }
    pub fn update_status(id: Id, status: &str) -> Self {
        Self {
            id,
            model_id: None,
            name: None,
            preview: None,
            description: None,
            view_count: None,
            like_count: None,
            hidden: None,
            year: None,
            price: None,
            height: None,
            top_tube_size: None,
            frame: None,
            seat_tube_sizes: None,
            headset: None,
            crankset: None,
            bottom_bracket: None,
            front_derail: None,
            rear_derail: None,
            brakes: None,
            shifters: None,
            brake_levers: None,
            saddle: None,
            seat_post: None,
            hubs: None,
            rims: None,
            handlebar: None,
            stem: None,
            status: Some(status.to_owned()),
        }
    }
}

pub struct BikeGetById {
    pub id: Id,
    pub fetch_deleted: bool,
    pub update_view_count: bool,
}

impl EntityById for BikeGetById {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        self.fetch_deleted
    }
}

impl BikeGetById {
    pub fn new(id: Id, fetch_deleted: bool, update_view_count: bool) -> Self {
        Self {
            id,
            fetch_deleted,
            update_view_count,
        }
    }

    pub fn new_admin(id: Id) -> Self {
        Self {
            id,
            fetch_deleted: true,
            update_view_count: false,
        }
    }
}

pub struct BikeCreateSessionKeys {
    pub bike_id: String,
}

impl BikeCreateSessionKeys {
    pub fn new(user_id: Id) -> Self {
        Self {
            bike_id: format!("bike_create_{user_id}_book_id"),
        }
    }
}

pub struct BikeDetailSessionKeys {
    pub visited: String,
}

impl BikeDetailSessionKeys {
    pub fn new(bike_id: Id) -> Self {
        Self {
            visited: format!("bike_{bike_id}_visited"),
        }
    }
}

pub struct BikeImageGetById {
    pub id: Id,
}

impl EntityById for BikeImageGetById {
    fn id(&self) -> Id {
        self.id
    }

    fn fetch_hidden(&self) -> bool {
        false
    }
}

impl BikeImageGetById {
    pub fn new(id: Id) -> Self {
        Self { id }
    }
}
