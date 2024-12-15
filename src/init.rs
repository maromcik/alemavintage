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
use crate::handlers::bike::{create_bike, create_bike_content, create_bike_page, get_bike_detail, get_bike_detail_content, upload_bike, upload_bike_form};
use crate::handlers::index::{index, index_content};
use crate::handlers::user::{user_manage_form_content, user_manage_form_page, user_manage_password, user_manage_password_form};

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
        .service(user_manage_form_content)
        .service(user_manage_password_form)
        .service(user_manage)
        .service(user_manage_password);

    let bike_scope = web::scope("bike")
        .app_data(web::Data::new(model_repository.clone()))
        .app_data(web::Data::new(brand_repository.clone()))
        .service(create_bike)
        .service(upload_bike)
        .service(create_bike_page)
        .service(create_bike_content)
    //     .service(edit_bike_page)
    //     .service(edit_bike_content)
    //     .service(edit_bike)
        .service(upload_bike_form)
    //     .service(get_bike)
    //     .service(manage_bike)
    //     .service(manage_bike_content)
    //     .service(releases_content)
    //     .service(releases_page)
    //     .service(remove_bike)
    //     .service(change_like)
    //     .service(search)
    //     .service(set_active_bike)
    //     .service(get_last_active_bike)
        .service(get_bike_detail)
        .service(get_bike_detail_content);
    //     .service(get_bike_player)
    //     .service(upload_book_cover)
    //     .service(upload_book_cover_post)
    //     .service(recommend_bikes)
    //     .service(restore_bike)
    //     .service(hard_remove_bike);
    //
    // let chapter_scope = web::scope("chapter")
    //     .app_data(web::Data::new(chapter_repository.clone()))
    //     .service(audio_selection_for_chapter)
    //     .service(get_chapter_timeline)
    //     .service(get_chapter_list)
    //     .service(create_chapter)
    //     .service(remove_chapter)
    //     .service(get_manage_chapter_list);
    //
    // let genre_scope = web::scope("genre")
    //     .app_data(web::Data::new(genre_repository.clone()))
    //     .app_data(web::Data::new(bike_repository.clone()))
    //     .service(get_genres_page)
    //     .service(get_genres_content)
    //     .service(get_bikes_by_genre)
    //     .service(get_bikes_by_genre_content);
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
            .service(index)
            .service(index_content)
            .service(studio::studio_index)
            .service(studio::studio_get_content)
            .service(ActixFiles::new("/media", "./media").prefer_utf8(true))
            .service(ActixFiles::new("/static", "./static").prefer_utf8(true));
    })
}
