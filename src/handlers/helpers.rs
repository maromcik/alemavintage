use crate::database::common::query_parameters::{DbColumn, DbOrder, DbOrderColumn, DbQueryParams};
use crate::database::common::{DbReadMany, DbReadOne};
use crate::database::models::bike::{BikeDetail, BikeSearch};
use crate::database::repositories::bike::repository::BikeRepository;
use crate::error::AppError;
use crate::templates::index::IndexBase;
use actix_web::web;
use crate::database::models::{GetById, Id};
use crate::templates::bike::BikeDetailBase;

// pub async fn get_releases(
//     u: Identity,
//     book_repo: web::Data<AudiobookRepository>,
// ) -> Result<Vec<AudiobookDisplay>, AppError> {
//     Ok(book_repo
//         .read_many(&AudiobookSearch::with_params(
//             DbQueryParams::state(BookState::Fresh(true)),
//             parse_user_id(u)?,
//         ))
//         .await?)
// }
// 
// pub async fn get_chapters_by_book(
//     chapter_repo: web::Data<ChapterRepository>,
//     audiobook_id: Id,
// ) -> Result<Vec<ChapterDisplay>, AppError> {
//     let displayed_chapters = get_displayable_chapters(chapter_repo, audiobook_id).await?;
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
//     audiobook_id: Id,
// ) -> Result<Vec<ChapterDisplay>, AppError> {
//     let chapters = chapter_repo
//         .read_many(&ChaptersGetByBookId::new(audiobook_id))
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
    // let user = user_repo
    //     .read_one(&UserGetById::new(&parse_user_id(u)?))
    //     .await?;
    let bikes = bike_repo
        .read_many(&BikeSearch::with_params(
            DbQueryParams::order(DbOrderColumn::new_column_only(DbColumn::ViewCount, DbOrder::Desc)),
        ))
        .await?;
    // let active_audiobooks = bike_repo
    //     .read_many(&AudiobookSearch::with_params(
    //         DbQueryParams::order(
    //             DbOrderColumn::new(DbTable::ActiveAudiobook, DbColumn::EditedAt, DbOrder::Desc),
    //             Some(BookState::Active(true)),
    //         ),
    //         user.id,
    //     ))
    //     .await?;
    // let finished_audiobooks = bike_repo
    //     .read_many(&AudiobookSearch::with_params(
    //         DbQueryParams::order(
    //             DbOrderColumn::new(DbTable::ActiveAudiobook, DbColumn::EditedAt, DbOrder::Desc),
    //             Some(BookState::Finished(true)),
    //         ),
    //         user.id,
    //     ))
    //     .await?;
    let template = IndexBase {
        email: "pes".to_string(),
        logged_in: true,
        bikes
    };
    Ok(template)
}

// pub async fn get_genre_base(
//     user: Identity,
//     audiobook_repo: web::Data<AudiobookRepository>,
//     genre_repo: web::Data<GenreRepository>,
//     genre_id: Id,
// ) -> Result<AudiobooksByGenreBase, AppError> {
//     let book_search = AudiobookSearch::search_by_genre_id(genre_id, parse_user_id(user)?);
//     let books = audiobook_repo
//         .read_many(&book_search)
//         .await?
//         .into_iter()
//         .map(AudiobookDisplay::from)
//         .collect();
//     let genre = genre_repo.read_one(&GenreGetById::new(&genre_id)).await?;
//     Ok(AudiobooksByGenreBase {
//         audiobooks: books,
//         genre,
//     })
// }
// 
// pub async fn get_library(
//     u: Identity,
//     book_repo: web::Data<AudiobookRepository>,
// ) -> Result<Vec<AudiobookDisplay>, AppError> {
//     Ok(book_repo.get_bookmarked(&parse_user_id(u)?).await?)
// }
// 
// pub async fn get_studio(
//     u: Identity,
//     book_repo: web::Data<AudiobookRepository>,
// ) -> Result<Vec<AudiobookDisplay>, AppError> {
//     let user_id = parse_user_id(u)?;
//     Ok(book_repo
//         .read_many(&AudiobookSearch::search_by_author_id(
//             user_id,
//             user_id,
//             DbQueryParams::deleted(),
//         ))
//         .await?)
// }
// 
// pub async fn get_author_profile(
//     user_id: Id,
//     book_repo: web::Data<AudiobookRepository>,
// ) -> Result<Vec<AudiobookDisplay>, AppError> {
//     Ok(book_repo
//         .read_many(&AudiobookSearch::search_by_author_id(
//             user_id,
//             user_id,
//             DbQueryParams::default(),
//         ))
//         .await?)
// }
// 
// pub async fn get_audiobook_edit(
//     u: Identity,
//     audiobook_repo: web::Data<AudiobookRepository>,
//     genre_repo: web::Data<GenreRepository>,
//     audiobook_id: Id,
// ) -> Result<AudiobookEditBase, AppError> {
//     let bike =
//         authorized_to_modify_join(&audiobook_repo, parse_user_id(u)?, audiobook_id).await?;
//     let genres = genre_repo.read_many(&GenreSearch::new(None)).await?;
//     Ok(AudiobookEditBase {
//         genres,
//         bike: AudiobookDisplay::from(bike),
//     })
// }
