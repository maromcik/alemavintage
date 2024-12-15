use crate::database::common::error::BackendErrorKind::{BikeDeleted, BikeDoesNotExist};
use crate::database::common::error::{DbResultMultiple, DbResultSingle, EntityError};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, EntityById,
    PoolHandler,
};
use serde::Serialize;

use sqlx::{Postgres, Transaction};

use crate::database::common::utilities::{entity_is_correct, generate_query_param_string};
use crate::database::models::bike::{
    Bike, BikeCreate, BikeDetail, BikeImage, BikeImageCreate, BikeImageSearch, BikeSearch,
};
use crate::database::models::{GetById, Id};

#[derive(Clone)]
pub struct BikeRepository {
    pool_handler: PoolHandler,
}

impl BikeRepository {
    pub async fn get_bike<'a>(
        params: &impl EntityById,
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

    async fn disconnect(&self) {
        self.pool_handler.disconnect().await;
    }
}

impl<ById> DbReadOne<ById, Bike> for BikeRepository
where
    ById: EntityById,
{
    async fn read_one(&self, params: &ById) -> DbResultSingle<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let bike = BikeRepository::get_bike(params, &mut transaction).await?;
        Ok(bike)
    }
}

impl<ById> DbReadOne<ById, BikeDetail> for BikeRepository
where
    ById: EntityById,
{
    async fn read_one(&self, params: &ById) -> DbResultSingle<BikeDetail> {
        let maybe_bike = sqlx::query_as!(
            BikeDetail,
            r#"
            SELECT
                bike.id,
                bike.brand_id,
                bike.model_id,
                bike.name,
                bike.description,
                bike.view_count,
                bike.like_count,
                bike.created_at,
                bike.edited_at,
                bike.deleted_at,

                brand.name as brand_name,
                model.name as model_name,
                
                image.path AS thumbnail
            FROM
                "Bike" AS bike
                    INNER JOIN
                "Brand" AS brand ON brand.id = bike.brand_id
                    INNER JOIN
                "Model" AS model ON model.id = bike.model_id
                    INNER JOIN
                "BikeImage" AS image ON image.bike_id = bike.id
            WHERE
                bike.id = $1
            "#,
            params.id(),
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let bike = entity_is_correct(
            maybe_bike,
            EntityError::new(BikeDeleted, BikeDoesNotExist),
            params.is_deleted(),
        )?;
        Ok(bike)
    }
}

impl DbReadMany<BikeSearch, BikeDetail> for BikeRepository {
    async fn read_many(&self, params: &BikeSearch) -> DbResultMultiple<BikeDetail> {
        let mut query = r#"
            SELECT
                bike.id,
                bike.brand_id,
                bike.model_id,
                bike.name,
                bike.description,
                bike.view_count,
                bike.like_count,
                bike.created_at,
                bike.edited_at,
                bike.deleted_at,

                brand.name AS brand_name,
                model.name AS model_name,
                
                image.path AS thumbnail
            FROM
                "Bike" AS bike
                    INNER JOIN
                "Brand" AS brand ON brand.id = bike.brand_id
                    INNER JOIN
                "Model" AS model ON model.id = bike.model_id
                    INNER JOIN
                "BikeImage" AS image ON image.bike_id = bike.id  
            WHERE
                image.ordering = 0 
                AND (bike.name = $1 OR $1 IS NULL)
                AND (bike.brand_id = $2 OR $2 IS NULL)
                AND (bike.model_id = $3 OR $3 IS NULL)
                AND (brand.name = $4 OR $4 IS NULL)
                AND (model.name = $5 OR $5 IS NULL)
            "#
        .to_owned();

        let query_params = generate_query_param_string(&params.query_params);
        query.push_str(query_params.as_str());

        let bikes = sqlx::query_as::<_, BikeDetail>(query.as_str())
            .bind(&params.name)
            .bind(params.brand_id)
            .bind(params.model_id)
            .bind(&params.brand_name)
            .bind(&params.model_name)
            .fetch_all(&self.pool_handler.pool)
            .await?;
        Ok(bikes)
    }
}

impl DbCreate<BikeCreate, Bike> for BikeRepository {
    async fn create(&self, params: &BikeCreate) -> DbResultSingle<Bike> {
        let book = sqlx::query_as!(
            Bike,
            r#"
            INSERT INTO "Bike" (name, brand_id, model_id, description)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            params.name,
            params.brand_id,
            params.model_id,
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
//                 brand_id = COALESCE($2, brand_id),
//                 model_id = COALESCE($3, model_id),
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
//             params.brand_id,
//             params.model_id,
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

impl<ById> DbDelete<ById, Bike> for BikeRepository
where
    ById: EntityById,
{
    async fn delete(&self, params: &ById) -> DbResultMultiple<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let _bike = BikeRepository::get_bike(params, &mut transaction).await?;

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

impl DbReadMany<BikeImageSearch, BikeImage> for BikeRepository {
    async fn read_many(&self, params: &BikeImageSearch) -> DbResultMultiple<BikeImage> {
        let mut query = r#"
            SELECT
                image.id,
                image.bike_id
                image.path,
                image.ordering,
            FROM
                "Bike" AS bike
                    INNER JOIN
                "BikeImage" AS image ON image.id = bike.model_id
            WHERE
                bike.id = $1 OR $1 IS NULL
            "#
        .to_owned();

        let query_params = generate_query_param_string(&params.query_params);
        query.push_str(query_params.as_str());

        let images = sqlx::query_as::<_, BikeImage>(query.as_str())
            .bind(&params.bike_id)
            .fetch_all(&self.pool_handler.pool)
            .await?;
        Ok(images)
    }
}

impl DbCreate<BikeImageCreate, Vec<BikeImage>> for BikeRepository {
    async fn create(&self, data: &BikeImageCreate) -> DbResultSingle<Vec<BikeImage>> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let mut images = Vec::default();
        for (i, path) in data.paths.iter().enumerate() {
            let bike_image = sqlx::query_as!(
                BikeImage,
                r#"
                    INSERT INTO "BikeImage" (bike_id, ordering, path)
                    VALUES ($1, $2, $3)
                    RETURNING *
                "#,
                data.bike_id,
                i as i32,
                path
            )
            .fetch_one(transaction.as_mut())
            .await?;
            images.push(bike_image);
        }
        transaction.commit().await?;
        Ok(images)
    }
}

impl<T> DbReadOne<T, BikeImage> for BikeRepository
where T: EntityById
{
    async fn read_one(&self, params: &T) -> DbResultSingle<BikeImage> {
        let maybe_image = sqlx::query_as!(
            BikeImage,
            r#"
            SELECT * FROM "BikeImage"
            WHERE bike_id = $1 and ordering = 0
            "#,
            params.id()
        )
            .fetch_optional(&self.pool_handler.pool)
            .await?;
        entity_is_correct(
            maybe_image,
            EntityError::new(BikeDeleted, BikeDoesNotExist),
            params.is_deleted(),
        )
    }
}