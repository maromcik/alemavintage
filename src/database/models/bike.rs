use crate::database::common::EntityById;
use crate::database::models::Id;
use crate::{database::common::query_parameters::DbQueryParams, error::AppError};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Bike {
    pub id: Id,
    // --------------
    pub name: String,
    pub internal_id: String,
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
    pub internal_id: String,
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
    pub internal_id: String,
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
            internal_id: value.internal_id,
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
            status: value.status.unwrap_or("<p>NO IMAGES FOUND</p>".to_string()),
            brand_name: value.brand_name,
            model_name: value.model_name,
            preview_path: value
                .preview_path
                .unwrap_or("/static/images/logo.png".to_string()),
            preview_width: value.preview_width.unwrap_or(400),
            preview_height: value.preview_height.unwrap_or(400),
            preview_thumbnail_path: value
                .preview_thumbnail_path
                .unwrap_or("/static/images/logo.png".to_string()),
        }
    }
}

impl BikeDisplay {
    pub fn description_to_markdown(mut self) -> Result<BikeDisplay, AppError> {
        self.description =
            markdown::to_html_with_options(&self.description, &markdown::Options::gfm())?;
        Ok(self)
    }
}

pub struct BikeCreate {
    pub name: String,
    pub internal_id: String,
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
        internal_id: &str,
        model_id: Id,
        preview: Option<Id>,
        description: &str,
        year: &i32,
        price: &i32,
    ) -> Self {
        Self {
            name: name.to_owned(),
            internal_id: internal_id.to_owned(),
            model_id,
            preview,
            description: description.to_owned(),
            year: *year,
            price: *price,
            height: 0,
            top_tube_size: 0,
            frame: String::default(),
            seat_tube_sizes: String::default(),
            headset: String::default(),
            crankset: String::default(),
            bottom_bracket: String::default(),
            front_derail: String::default(),
            rear_derail: String::default(),
            brakes: String::default(),
            shifters: String::default(),
            brake_levers: String::default(),
            saddle: String::default(),
            seat_post: String::default(),
            hubs: String::default(),
            rims: String::default(),
            handlebar: String::default(),
            stem: String::default(),
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

pub struct BikeUpdate {
    pub id: Id,
    pub internal_id: Option<String>,
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
        internal_id: Option<&str>,
        name: Option<&str>,
        model_id: Option<&Id>,
        preview: Option<&Id>,
        description: Option<&str>,
        view_count: Option<&i64>,
        like_count: Option<&i64>,
        hidden: Option<&bool>,
        year: Option<&i32>,
        price: Option<&i32>,
        status: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            internal_id: internal_id.and_then(change_to_owned),
            name: name.and_then(change_to_owned),
            model_id: model_id.copied(),
            preview: preview.copied(),
            description: description.and_then(change_to_owned),
            view_count: view_count.copied(),
            like_count: like_count.copied(),
            hidden: hidden.copied(),
            year: year.copied(),
            price: price.copied(),
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
            status: status.and_then(change_to_owned),
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none()
            && self.internal_id.is_none()
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
            internal_id: None,
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
            internal_id: None,
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
            internal_id: None,
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
            internal_id: None,
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
