use askama::Template;
use crate::database::models::brand::Brand;
use crate::database::models::model::Model;

#[derive(Template)]
#[template(path = "brand/create/page.html")]
pub struct BrandCreatePageTemplate {
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "brand/create/content.html")]
pub struct BrandCreateContentTemplate {
    pub logged_in: bool,
}
#[derive(Template)]
#[template(path = "brand/page.html")]
pub struct BrandPageTemplate {
    pub brands: Vec<Brand>,
    pub logged_in: bool,
}

#[derive(Template)]
#[template(path = "brand/content.html")]
pub struct BrandContentTemplate {
    pub brands: Vec<Brand>,
    pub logged_in: bool,
}

// #[derive(Template)]
// #[template(path = "studio_edit_brand.html")]
// pub struct BikeEditPageTemplate {
//     pub brand: Brand,
//     pub logged_in: bool,
// }
//
// #[derive(Template)]
// #[template(path = "brand/brand_edit.html")]
// pub struct BikeEditContentTemplate {
//     pub brand: BikeDetail,
//     pub brands: Vec<Brand>,
//     pub models: Vec<Model>,
//     pub logged_in: bool,
// }
//
// pub struct BikeDetailBase {
//     pub brand: BikeDetail,
//     pub brand_images: Vec<String>,
//     pub logged_in: bool,
// }


// #[derive(Template)]
// #[template(path = "detail.html")]
// pub struct BikeDetailPageTemplate {
//     pub brand: BikeDetail,
//     pub brand_images: Vec<String>,
//     pub logged_in: bool,
// }
//
// #[derive(Template)]
// #[template(path = "brand/detail-content.html")]
// pub struct BikeDetailContentTemplate {
//     pub brand: BikeDetail,
//     pub brand_images: Vec<String>,
//     pub logged_in: bool,
// }
//
// impl From<BikeDetailBase> for BikeDetailPageTemplate {
//     fn from(value: BikeDetailBase) -> Self {
//         Self {
//             brand: value.brand,
//             brand_images: value.brand_images,
//             logged_in: value.logged_in,
//         }
//     }
// }
//
// impl From<BikeDetailBase> for BikeDetailContentTemplate {
//     fn from(value: BikeDetailBase) -> Self {
//         Self {
//             brand: value.brand,
//             brand_images: value.brand_images,
//             logged_in: value.logged_in,
//         }
//     }
// }
//
//
// #[derive(Template)]
// #[template(path = "detail_admin.html")]
// pub struct BikeDetailAdminPageTemplate {
//     pub brand: BikeDetail,
//     pub brand_images: Vec<String>,
//     pub logged_in: bool,
// }
//
// #[derive(Template)]
// #[template(path = "brand/detail_admin-content.html")]
// pub struct BikeDetailAdminContentTemplate {
//     pub brand: BikeDetail,
//     pub brand_images: Vec<String>,
//     pub logged_in: bool,
// }
//
// impl From<BikeDetailBase> for BikeDetailAdminPageTemplate {
//     fn from(value: BikeDetailBase) -> Self {
//         Self {
//             brand: value.brand,
//             brand_images: value.brand_images,
//             logged_in: value.logged_in,
//         }
//     }
// }
//
// impl From<BikeDetailBase> for BikeDetailAdminContentTemplate {
//     fn from(value: BikeDetailBase) -> Self {
//         Self {
//             brand: value.brand,
//             brand_images: value.brand_images,
//             logged_in: value.logged_in,
//         }
//     }
// }
//
// #[derive(Template)]
// #[template(path = "brand/brand_upload.html")]
// pub struct BikeUploadFormTemplate {
//     pub message: String,
// }