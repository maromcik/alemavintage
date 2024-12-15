use crate::database::common::PoolHandler;
use crate::database::common::{DbPoolHandler, DbRepository};

use crate::handlers::*;
use actix_files::Files as ActixFiles;
use actix_web::web;
use actix_web::web::{service, ServiceConfig};
use sqlx::PgPool;
use crate::database::repositories::bike::repository::BikeRepository;
use crate::database::repositories::user::repository::UserRepository;
use crate::handlers::bike::{get_bike_detail, get_bike_detail_content, upload_bike, upload_bike_form};
use crate::handlers::index::{index, index_content};
use crate::handlers::user::{user_manage_form_content, user_manage_form_page, user_manage_password, user_manage_password_form};

pub fn configure_webapp(pool: &PgPool) -> Box<dyn FnOnce(&mut ServiceConfig)> {
    let user_repo = UserRepository::new(PoolHandler::new(pool.clone()));
    let bike_repo = BikeRepository::new(PoolHandler::new(pool.clone()));
    // let chapter_repository = ChapterRepository::new(PoolHandler::new(pool.clone()));
    // let genre_repository = GenreRepository::new(PoolHandler::new(pool.clone()));
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
    //     .app_data(web::Data::new(genre_repository.clone()))
    //     .app_data(web::Data::new(chapter_repository.clone()))
    //     .service(create_audiobook)
        .service(upload_bike)
    //     .service(create_audiobook_page)
    //     .service(create_audiobook_content)
    //     .service(edit_audiobook_page)
    //     .service(edit_audiobook_content)
    //     .service(edit_audiobook)
        .service(upload_bike_form)
    //     .service(get_audiobook)
    //     .service(manage_audiobook)
    //     .service(manage_audiobook_content)
    //     .service(releases_content)
    //     .service(releases_page)
    //     .service(remove_audiobook)
    //     .service(change_like)
    //     .service(search)
    //     .service(set_active_audiobook)
    //     .service(get_last_active_audiobook)
        .service(get_bike_detail)
        .service(get_bike_detail_content);
    //     .service(get_audiobook_player)
    //     .service(upload_book_cover)
    //     .service(upload_book_cover_post)
    //     .service(recommend_audiobooks)
    //     .service(restore_audiobook)
    //     .service(hard_remove_audiobook);
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
    //     .app_data(web::Data::new(audiobook_repository.clone()))
    //     .service(get_genres_page)
    //     .service(get_genres_content)
    //     .service(get_audiobooks_by_genre)
    //     .service(get_audiobooks_by_genre_content);
    //
    // let rating_scope = web::scope("rating")
    //     .app_data(web::Data::new(rating_repository.clone()))
    //     .service(create_rating)
    //     .service(get_ratings_by_audiobook)
    //     .service(get_my_rating)
    //     .service(get_rating_summary)
    //     .service(get_pagination)
    //     .service(remove_rating_for_audiobook);
    //
    Box::new(move |cfg: &mut ServiceConfig| {
        cfg
            .app_data(web::Data::new(bike_repo.clone()))
            .app_data(web::Data::new(user_repo.clone()))
            .service(bike_scope)
            .service(user_scope)
            .service(index)
            .service(index_content)
            .service(ActixFiles::new("/media", "./media").prefer_utf8(true))
            .service(ActixFiles::new("/static", "./static").prefer_utf8(true));
    })
}
