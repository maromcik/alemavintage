use crate::database::common::PoolHandler;
use crate::database::common::{DbPoolHandler, DbRepository};

use crate::handlers::*;
use actix_files::Files as ActixFiles;
use actix_web::web;
use actix_web::web::{service, ServiceConfig};
use sqlx::PgPool;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::brand::repository::BrandRepository;
use crate::database::repositories::model::repository::ModelRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::handlers::bike::{create_bike, create_bike_page, edit_bike, edit_bike_page, get_bike_detail, get_bikes, hard_remove_bike, manage_bike, remove_bike, restore_bike, upload_bike, upload_bike_form};
use crate::handlers::brand::{create_brand, create_brand_page, get_brands};
use crate::handlers::index::{index};
use crate::handlers::user::{user_manage_form_page, user_manage_password, user_manage_password_form};

pub fn configure_webapp(pool: &PgPool) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    let user_repo = UserRepository::new(PoolHandler::new(pool.clone()));
    let bike_repo = BikeRepository::new(PoolHandler::new(pool.clone()));
    let model_repository = ModelRepository::new(PoolHandler::new(pool.clone()));
    let brand_repository = BrandRepository::new(PoolHandler::new(pool.clone()));
    // let rating_repository = RatingRepository::new(PoolHandler::new(pool.clone()));
    let user_scope = web::scope("user")
        .service(user_login_page)
        .service(user_login)
        .service(user_logout)
        .service(user_manage_form_page)
        .service(user_manage_password_form)
        .service(user_manage)
        .service(user_manage_password);

    let bike_scope = web::scope("bike")
        .app_data(web::Data::new(model_repository.clone()))
        .app_data(web::Data::new(brand_repository.clone()))
        .service(get_bikes)
        .service(create_bike)
        .service(upload_bike)
        .service(create_bike_page)
        .service(edit_bike)
        .service(edit_bike_page)
        .service(upload_bike_form)
    //     .service(get_bike)
        .service(manage_bike)
    //     .service(releases_content)
    //     .service(releases_page)
        .service(remove_bike)
    //     .service(change_like)
    //     .service(search)
    //     .service(set_active_bike)
    //     .service(get_last_active_bike)
        .service(get_bike_detail)
    //     .service(get_bike_player)
    //     .service(upload_book_cover)
    //     .service(upload_book_cover_post)
    //     .service(recommend_bikes)
        .service(restore_bike)
        .service(hard_remove_bike);

    let brand_scope = web::scope("brand")
        .app_data(web::Data::new(brand_repository.clone()))
        .service(create_brand)
        .service(get_brands)
        .service(create_brand_page);

    let model_scope = web::scope("model")
        .app_data(web::Data::new(brand_repository.clone()))
        .service(create_brand_page);
    //
    // let rating_scope = web::scope("rating")
    //     .app_data(web::Data::new(rating_repository.clone()))
    //     .service(create_rating)
    //     .service(get_ratings_by_bike)
    //     .service(get_my_rating)
    //     .service(get_rating_summary)
    //     .service(get_pagination)
    //     .service(remove_rating_for_bike);
    //
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg
            .app_data(web::Data::new(bike_repo.clone()))
            .app_data(web::Data::new(user_repo.clone()))
            .service(bike_scope)
            .service(user_scope)
            .service(brand_scope)
            .service(index)
            .service(studio::studio_index)
            .service(ActixFiles::new("/media", "./media").prefer_utf8(true))
            .service(ActixFiles::new("/static", "./static").prefer_utf8(true));
    })
}
