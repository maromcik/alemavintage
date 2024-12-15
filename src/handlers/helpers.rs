use actix_identity::Identity;
use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams};
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::bike::{BikeDetail, BikeSearch};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::templates::index::IndexBase;
use actix_web::web;
use crate::database::models::{GetById, Id};
use crate::handlers::utilities::parse_user_id;
use crate::templates::bike::BikeDetailBase;

// pub async fn get_releases(
//     u: Identity,
//     book_repo: web::Data<BikeRepository>,
// ) -> Result<Vec<BikeDisplay>, AppError> {
//     Ok(book_repo
//         .read_many(&BikeSearch::with_params(
//             DbQueryParams::state(BookState::Fresh(true)),
//             parse_user_id(u)?,
//         ))
//         .await?)
// }
// 
// pub async fn get_chapters_by_book(
//     chapter_repo: web::Data<ChapterRepository>,
//     bike_id: Id,
// ) -> Result<Vec<ChapterDisplay>, AppError> {
//     let displayed_chapters = get_displayable_chapters(chapter_repo, bike_id).await?;
//     Ok(displayed_chapters)
// }
// 
pub async fn get_bike_detail_base(
    bike_repo: web::Data<BikeRepository>,
    bike_id: Id,
) -> Result<BikeDetailBase, AppError> {
    let bike: BikeDetail = <BikeRepository as DbReadOne<GetById, BikeDetail>>::read_one(
        bike_repo.as_ref(),
        &GetById::new(bike_id),
    )
        .await?;
    
    Ok(BikeDetailBase {
        bike,
    })
}
// 
// pub async fn get_displayable_chapters(
//     chapter_repo: web::Data<ChapterRepository>,
//     bike_id: Id,
// ) -> Result<Vec<ChapterDisplay>, AppError> {
//     let chapters = chapter_repo
//         .read_many(&ChaptersGetByBookId::new(bike_id))
//         .await?;
//     Ok(chapters
//         .into_iter()
//         .enumerate()
//         .map(|(order, ch)| ChapterDisplay {
//             id: ch.id,
//             name: ch.name,
//             order: order + 1,
//             position: ch.position,
//         })
//         .collect())
// }

pub async fn get_index_base(
    bike_repo: web::Data<BikeRepository>,
) -> Result<IndexBase, AppError> {

    let bikes = bike_repo
        .read_many(&BikeSearch::with_params(
            DbQueryParams::order(DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc)),
        ))
        .await?;

    let template = IndexBase {
        logged_in: true,
        bikes,
    };
    Ok(template)
}

// pub async fn get_genre_base(
//     user: Identity,
//     bike_repo: web::Data<BikeRepository>,
//     genre_repo: web::Data<GenreRepository>,
//     genre_id: Id,
// ) -> Result<BikesByGenreBase, AppError> {
//     let book_search = BikeSearch::search_by_genre_id(genre_id, parse_user_id(user)?);
//     let books = bike_repo
//         .read_many(&book_search)
//         .await?
//         .into_iter()
//         .map(BikeDisplay::from)
//         .collect();
//     let genre = genre_repo.read_one(&GenreGetById::new(&genre_id)).await?;
//     Ok(BikesByGenreBase {
//         bikes: books,
//         genre,
//     })
// }
// 
// pub async fn get_library(
//     u: Identity,
//     book_repo: web::Data<BikeRepository>,
// ) -> Result<Vec<BikeDisplay>, AppError> {
//     Ok(book_repo.get_bookmarked(&parse_user_id(u)?).await?)
// }
// 
pub async fn get_studio(
    book_repo: web::Data<BikeRepository>,
) -> Result<Vec<BikeDetail>, AppError> {
    Ok(book_repo
        .read_many(&BikeSearch::with_params(DbQueryParams::deleted()))
        .await?)
    
}
// 
// pub async fn get_author_profile(
//     user_id: Id,
//     book_repo: web::Data<BikeRepository>,
// ) -> Result<Vec<BikeDisplay>, AppError> {
//     Ok(book_repo
//         .read_many(&BikeSearch::search_by_author_id(
//             user_id,
//             user_id,
//             DbQueryParams::default(),
//         ))
//         .await?)
// }
// 
// pub async fn get_bike_edit(
//     u: Identity,
//     bike_repo: web::Data<BikeRepository>,
//     genre_repo: web::Data<GenreRepository>,
//     bike_id: Id,
// ) -> Result<BikeEditBase, AppError> {
//     let bike =
//         authorized_to_modify_join(&bike_repo, parse_user_id(u)?, bike_id).await?;
//     let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
//     Ok(BikeEditBase {
//         genres,
//         bike: BikeDisplay::from(bike),
//     })
// }
