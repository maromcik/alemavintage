use crate::database::common::PoolHandler;
use crate::database::common::{DbPoolHandler, DbRepository};
use std::sync::Arc;

use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::handlers::bike::{create_bike, create_bike_page, edit_bike, edit_bike_page, get_bike_detail, get_bikes, hard_remove_bike, manage_bike, remove_bike, restore_bike, upload_bike, upload_bike_form};
use crate::handlers::brand::{create_brand, create_brand_page, get_brands};
use crate::handlers::index::index;
use crate::handlers::user::{user_manage_form_page, user_manage_password, user_manage_password_form};
use crate::handlers::*;
use crate::AppState;
use actix_files::Files as ActixFiles;
use actix_web::web;
use actix_web::web::ServiceConfig;
use minijinja_autoreload::AutoReloader;
use sqlx::PgPool;
use crate::handlers::model::{create_model, create_model_page, get_model, get_models};

pub fn configure_webapp(pool: &PgPool, jinja: Arc<AutoReloader>) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    let user_repo = UserRepository::new(PoolHandler::new(pool.clone()));
    let bike_repo = BikeRepository::new(PoolHandler::new(pool.clone()));
    let model_repository = ModelRepository::new(PoolHandler::new(pool.clone()));
    let brand_repository = BrandRepository::new(PoolHandler::new(pool.clone()));
    
    let studio_scope = web::scope("studio")
        .app_data(web::Data::new(bike_repo.clone()))
        .service(studio::studio_index);
    
    let user_scope = web::scope("user")
        .app_data(web::Data::new(user_repo.clone()))
        .service(user_login_page)
        .service(user_login)
        .service(user_logout)
        .service(user_manage_form_page)
        .service(user_manage_password_form)
        .service(user_manage)
        .service(user_manage_password);

    let bike_scope = web::scope("bike")
        .app_data(web::Data::new(bike_repo.clone()))
        .app_data(web::Data::new(model_repository.clone()))
        .app_data(web::Data::new(brand_repository.clone()))
        .service(get_bikes)
        .service(create_bike)
        .service(upload_bike)
        .service(create_bike_page)
        .service(edit_bike)
        .service(edit_bike_page)
        .service(upload_bike_form)
        .service(manage_bike)
        .service(remove_bike)
        .service(get_bike_detail)
        .service(restore_bike)
        .service(hard_remove_bike);

    let brand_scope = web::scope("brand")
        .app_data(web::Data::new(brand_repository.clone()))
        .service(create_brand)
        .service(get_brands)
        .service(create_brand_page);

    let model_scope = web::scope("model")
        .app_data(web::Data::new(bike_repo.clone()))
        .app_data(web::Data::new(model_repository.clone()))
        .app_data(web::Data::new(brand_repository.clone()))
        .service(create_model)
        .service(get_models)
        .service(get_model)
        .service(create_model_page);
    

    Box::new(move |cfg: &mut ServiceConfig| {
        cfg
            .app_data(web::Data::new(user_repo.clone()))
            .app_data(web::Data::new(AppState::new(jinja.clone())))
            .service(studio_scope)
            .service(bike_scope)
            .service(user_scope)
            .service(brand_scope)
            .service(model_scope)
            .service(index)
            .service(ActixFiles::new("/media", "./media").prefer_utf8(true))
            .service(ActixFiles::new("/static", "./static").prefer_utf8(true));
    })
}
