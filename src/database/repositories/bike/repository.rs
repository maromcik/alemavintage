use crate::database::common::error::BackendErrorKind::{
    BikeDeleted, BikeDoesNotExist,
};
use crate::database::common::error::{
    DbResultMultiple, DbResultSingle, EntityError,
};
use crate::database::common::{DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, EntityById, PoolHandler};


use sqlx::{Postgres, Transaction};

use crate::database::common::utilities::entity_is_correct;
use crate::database::models::bike::{Bike, BikeCreate};
use crate::database::models::{GetById, Id};

#[derive(Clone)]
pub struct BikeRepository {
    pool_handler: PoolHandler,
}

impl BikeRepository {
    pub async fn get_bike<'a>(
        params: &GetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Bike> {
        let maybe_bike = sqlx::query_as!(
            Bike,
            r#"
            SELECT * FROM "Bike"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;
        entity_is_correct(
            maybe_bike,
            EntityError::new(BikeDeleted, BikeDoesNotExist),
            params.is_deleted(),
        )
    }

    pub async fn increment_view_count<'a>(
        book_id: &Id,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<()> {
        sqlx::query!(
            r#"
            UPDATE "Bike"
            SET view_count = view_count + 1
            WHERE id = $1
            "#,
            book_id,
        )
        .execute(transaction_handle.as_mut())
        .await?;
        Ok(())
    }


    pub async fn restore(&self, params: &impl EntityById) -> DbResultMultiple<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let books = sqlx::query_as!(
            Bike,
            r#"
            UPDATE "Bike" SET
                deleted_at = null,
                edited_at = current_timestamp
            WHERE id = $1
            RETURNING *
            "#,
            params.id(),
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(books)
    }

    pub async fn hard_delete(&self, params: &impl EntityById) -> DbResultMultiple<Bike> {
        let books = sqlx::query_as!(
            Bike,
            r#"
            DELETE FROM "Bike"
            WHERE id = $1
            RETURNING *
            "#,
            params.id(),
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(books)
    }
}


impl DbRepository for BikeRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&self) -> () {
        self.pool_handler.disconnect().await;
    }
}


impl DbReadOne<GetById, Bike> for BikeRepository {
    async fn read_one(&self, params: &GetById) -> DbResultSingle<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let bike = BikeRepository::get_bike(params, &mut transaction).await?;
        Ok(bike)
    }
}


impl DbCreate<BikeCreate, Bike> for BikeRepository {
    async fn create(&self, params: &BikeCreate) -> DbResultSingle<Bike> {
        let book = sqlx::query_as!(
            Bike,
            r#"
            INSERT INTO "Bike" (name, brand_id, model_id, thumbnail, description)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            params.name,
            params.brand_id,
            params.model_id,
            params.thumbnail,
            params.description
        )
            .fetch_one(&self.pool_handler.pool)
            .await?;

        Ok(book)
    }
}

// #[async_trait]
// impl DbUpdate<BikeUpdate, Bike> for BikeRepository {
//     async fn update(&self, params: &BikeUpdate) -> DbResultMultiple<Bike> {
//         if params.update_fields_none() {
//             return Err(DbError::from(BackendError::new(
//                 BikeUpdateParametersEmpty,
//             )));
//         }
//
//         let mut transaction = self.pool_handler.pool.begin().await?;
//         let bike = BikeRepository::get_bike(
//             &GetById {
//                 id: params.id,
//                 fetch_deleted: true,
//             },
//             &mut transaction,
//         )
//         .await?;
//         let updated_audio_books = sqlx::query_as!(
//             Bike,
//             r#"
//             UPDATE "Bike"
//             SET
//                 name = COALESCE($1, name),
//                 author_id = COALESCE($2, author_id),
//                 genre_id = COALESCE($3, genre_id),
//                 file_path = COALESCE($4, file_path),
//                 length = COALESCE($5, length),
//                 stream_count = COALESCE($6, stream_count),
//                 like_count = COALESCE($7, like_count),
//                 overall_rating = COALESCE($8, overall_rating),
//                 thumbnail = COALESCE($9, thumbnail),
//                 description = COALESCE($10, description),
//                 edited_at = current_timestamp
//             WHERE id = $11
//             RETURNING *
//             "#,
//             params.name,
//             params.author_id,
//             params.genre_id,
//             params.file_path,
//             params.length,
//             params.stream_count,
//             params.like_count,
//             params.overall_rating,
//             params.thumbnail,
//             params.description,
//             bike.id
//         )
//         .fetch_all(transaction.as_mut())
//         .await?;
//         transaction.commit().await?;
//
//         Ok(updated_audio_books)
//     }
// }


impl DbDelete<GetById, Bike> for BikeRepository {
    async fn delete(&self, params: &GetById) -> DbResultMultiple<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let _bike = BikeRepository::get_bike(
            &params,
            &mut transaction,
        )
        .await?;

        let books = sqlx::query_as!(
            Bike,
            r#"
            UPDATE "Bike" SET
                deleted_at = current_timestamp,
                edited_at = current_timestamp
            WHERE id = $1
            RETURNING *
            "#,
            params.id(),
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        Ok(books)
    }
}
