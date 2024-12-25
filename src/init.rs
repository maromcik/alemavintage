use crate::database::common::PoolHandler;
use crate::database::common::{DbPoolHandler, DbRepository};

use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::handlers::bike::{clone_bike, create_bike, create_bike_page, edit_bike, edit_bike_page, get_bike_detail, get_bikes, hide_bike, remove_bike, restore_bike, reupload_bike, reupload_bike_page, upload_bike, upload_bike_page, upload_bike_thumbnail, upload_bike_thumbnail_page};
use crate::handlers::brand::{
    create_brand, create_brand_page, edit_brand, edit_brand_page, get_brand, get_brands,
    remove_brand,
};
use crate::handlers::index::{about, index};
use crate::handlers::model::{
    create_model, create_model_page, edit_model, edit_model_page, get_model, get_models,
    remove_model,
};
use crate::handlers::user::{
    contact_admin_bike, contact_admin_general, login, login_user, logout_user, user_manage,
    user_manage_form_page, user_manage_password, user_manage_password_form,
};
use crate::utils::AppState;
use actix_files::Files as ActixFiles;
use actix_web::web;
use actix_web::web::ServiceConfig;
use sqlx::PgPool;
use crate::database::repositories::tag::repository::TagRepository;
use crate::handlers::tag::{create_tag, get_tag};

pub fn configure_webapp(pool: &PgPool, app_state: AppState) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    let user_repo = UserRepository::new(PoolHandler::new(pool.clone()));
    let bike_repo = BikeRepository::new(PoolHandler::new(pool.clone()));
    let model_repository = ModelRepository::new(PoolHandler::new(pool.clone()));
    let brand_repository = BrandRepository::new(PoolHandler::new(pool.clone()));
    let tag_repository = TagRepository::new(PoolHandler::new(pool.clone()));

    let user_scope = web::scope("user")
        .app_data(web::Data::new(user_repo.clone()))
        .app_data(web::Data::new(bike_repo.clone()))
        .service(login)
        .service(login_user)
        .service(logout_user)
        .service(user_manage_form_page)
        .service(user_manage_password_form)
        .service(user_manage)
        .service(user_manage_password)
        .service(contact_admin_bike)
        .service(contact_admin_general);

    let bike_scope = web::scope("bike")
        .app_data(web::Data::new(bike_repo.clone()))
        .app_data(web::Data::new(model_repository.clone()))
        .app_data(web::Data::new(brand_repository.clone()))
        .app_data(web::Data::new(tag_repository.clone()))
        .service(get_bikes)
        .service(create_bike)
        .service(upload_bike)
        .service(create_bike_page)
        .service(edit_bike)
        .service(edit_bike_page)
        .service(upload_bike_page)
        .service(hide_bike)
        .service(get_bike_detail)
        .service(restore_bike)
        .service(upload_bike_thumbnail_page)
        .service(upload_bike_thumbnail)
        .service(reupload_bike_page)
        .service(reupload_bike)
        .service(remove_bike)
        .service(clone_bike);

    let brand_scope = web::scope("brand")
        .app_data(web::Data::new(bike_repo.clone()))
        .app_data(web::Data::new(brand_repository.clone()))
        .app_data(web::Data::new(model_repository.clone()))
        .service(create_brand_page)
        .service(create_brand)
        .service(get_brands)
        .service(get_brand)
        .service(edit_brand)
        .service(edit_brand_page)
        .service(remove_brand);

    let model_scope = web::scope("model")
        .app_data(web::Data::new(bike_repo.clone()))
        .app_data(web::Data::new(model_repository.clone()))
        .app_data(web::Data::new(brand_repository.clone()))
        .service(create_model_page)
        .service(create_model)
        .service(get_models)
        .service(get_model)
        .service(edit_model)
        .service(edit_model_page)
        .service(remove_model);

    let tag_scope = web::scope("tag")
        .app_data(web::Data::new(bike_repo.clone()))
        .app_data(web::Data::new(tag_repository.clone()))
        .service(get_tag)
        .service(create_tag);

    Box::new(move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(app_state))
            .service(bike_scope)
            .service(user_scope)
            .service(brand_scope)
            .service(model_scope)
            .service(tag_scope)
            .service(index)
            .service(about)
            .service(ActixFiles::new("/media", "./media").prefer_utf8(true))
            .service(ActixFiles::new("/static", "./static").prefer_utf8(true));
    })
}
