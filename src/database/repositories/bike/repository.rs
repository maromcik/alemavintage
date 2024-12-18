use crate::database::common::error::BackendErrorKind::{
    BikeDeleted, BikeDoesNotExist, BikeUpdateParametersEmpty,
};
use crate::database::common::error::{
    BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError,
};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, EntityById,
    PoolHandler,
};

use sqlx::{Postgres, Transaction};

use crate::database::common::utilities::{entity_is_correct, generate_query_param_string};
use crate::database::models::bike::{
    Bike, BikeCreate, BikeDetail, BikeGetById, BikeImage, BikeImageCreate, BikeImageSearch,
    BikeSearch, BikeUpdate,
};
use crate::database::models::GetById;

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
            params.fetch_deleted(),
        )
    }

    pub async fn increment_view_count<'a>(
        params: &impl EntityById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<()> {
        sqlx::query!(
            r#"
            UPDATE "Bike"
            SET view_count = view_count + 1
            WHERE id = $1
            "#,
            params.id(),
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
        let bikes = sqlx::query_as!(
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
        Ok(bikes)
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

impl DbReadOne<BikeGetById, BikeDetail> for BikeRepository {
    async fn read_one(&self, params: &BikeGetById) -> DbResultSingle<BikeDetail> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        if params.update_view_count {
            BikeRepository::increment_view_count(params, &mut transaction).await?;
        }

        let maybe_bike = sqlx::query_as!(
            BikeDetail,
            r#"
            SELECT
                bike.id,
                bike.model_id,
                bike.name,
                bike.thumbnail,
                bike.description,
                bike.view_count,
                bike.like_count,
                bike.created_at,
                bike.edited_at,
                bike.deleted_at,
                
                brand.id as brand_id,
                brand.name as brand_name,
                model.name as model_name                
            FROM
                "Bike" AS bike
                    INNER JOIN
                "Model" AS model ON model.id = bike.model_id
                    INNER JOIN
                "Brand" AS brand ON brand.id = model.brand_id
            WHERE
                bike.id = $1
            "#,
            params.id(),
        )
        .fetch_optional(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        let bike = entity_is_correct(
            maybe_bike,
            EntityError::new(BikeDeleted, BikeDoesNotExist),
            params.fetch_deleted(),
        )?;
        Ok(bike)
    }
}

impl DbReadMany<BikeSearch, BikeDetail> for BikeRepository {
    async fn read_many(&self, params: &BikeSearch) -> DbResultMultiple<BikeDetail> {
        let mut query = r#"
            SELECT
                bike.id,
                bike.model_id,
                bike.name,
                bike.thumbnail,
                bike.description,
                bike.view_count,
                bike.like_count,
                bike.created_at,
                bike.edited_at,
                bike.deleted_at,

                brand.id   AS brand_id,
                brand.name AS brand_name,
                model.name AS model_name
            FROM
                "Bike" AS bike
                    INNER JOIN
                "Model" AS model ON model.id = bike.model_id
                    INNER JOIN
                "Brand" AS brand ON brand.id = model.brand_id  
            WHERE
                (bike.name = $1 OR $1 IS NULL)
                AND (brand_id = $2 OR $2 IS NULL)
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
            INSERT INTO "Bike" (name, model_id, thumbnail, description)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            params.name,
            params.model_id,
            params.thumbnail,
            params.description
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(book)
    }
}

impl DbUpdate<BikeUpdate, Bike> for BikeRepository {
    async fn update(&self, params: &BikeUpdate) -> DbResultMultiple<Bike> {
        if params.update_fields_none() {
            return Err(DbError::from(BackendError::new(BikeUpdateParametersEmpty)));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;
        let bike = BikeRepository::get_bike(
            &GetById {
                id: params.id,
                fetch_deleted: true,
            },
            &mut transaction,
        )
        .await?;
        let updated_audio_books = sqlx::query_as!(
            Bike,
            r#"
            UPDATE "Bike"
            SET
                name = COALESCE($1, name),
                model_id = COALESCE($2, model_id),
                thumbnail = COALESCE($3, thumbnail),
                description = COALESCE($4, description),
                view_count = COALESCE($5, view_count),
                like_count = COALESCE($6, like_count),
                edited_at = current_timestamp
            WHERE id = $7
            RETURNING *
            "#,
            params.name,
            params.model_id,
            params.thumbnail,
            params.description,
            params.view_count,
            params.like_count,
            bike.id,
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;

        Ok(updated_audio_books)
    }
}

impl<ById> DbDelete<ById, Bike> for BikeRepository
where
    ById: EntityById,
{
    async fn delete(&self, params: &ById) -> DbResultMultiple<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let _bike = BikeRepository::get_bike(params, &mut transaction).await?;

        let bikes = sqlx::query_as!(
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

        Ok(bikes)
    }
}

impl DbReadMany<BikeImageSearch, BikeImage> for BikeRepository {
    async fn read_many(&self, params: &BikeImageSearch) -> DbResultMultiple<BikeImage> {
        let mut query = r#"
            SELECT
                image.id,
                image.bike_id,
                image.path
            FROM
                "BikeImage" AS image
            WHERE
                image.bike_id = $1 OR $1 IS NULL
            "#
        .to_owned();

        let query_params = generate_query_param_string(&params.query_params);
        query.push_str(query_params.as_str());

        let images = sqlx::query_as::<_, BikeImage>(query.as_str())
            .bind(params.bike_id)
            .fetch_all(&self.pool_handler.pool)
            .await?;
        Ok(images)
    }
}

impl DbCreate<BikeImageCreate, Vec<BikeImage>> for BikeRepository {
    async fn create(&self, data: &BikeImageCreate) -> DbResultSingle<Vec<BikeImage>> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let mut images = Vec::default();
        for path in data.paths.iter() {
            let bike_image = sqlx::query_as!(
                BikeImage,
                r#"
                    INSERT INTO "BikeImage" (bike_id, path)
                    VALUES ($1, $2)
                    RETURNING *
                "#,
                data.bike_id,
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
where
    T: EntityById,
{
    async fn read_one(&self, params: &T) -> DbResultSingle<BikeImage> {
        let maybe_image = sqlx::query_as!(
            BikeImage,
            r#"
            SELECT * FROM "BikeImage"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;
        entity_is_correct(
            maybe_image,
            EntityError::new(BikeDeleted, BikeDoesNotExist),
            params.fetch_deleted(),
        )
    }
}
