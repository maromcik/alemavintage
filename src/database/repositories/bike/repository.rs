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
    Bike, BikeCreate, BikeDetail, BikeGetById, BikeSearch, BikeUpdate,
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
            params.fetch_hidden(),
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

    #[allow(dead_code)]
    pub async fn unlink_preview(&self, params: &impl EntityById) -> DbResultMultiple<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let bikes = sqlx::query_as!(
            Bike,
            r#"
            UPDATE "Bike" SET
                preview = NULL
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

    pub async fn restore(&self, params: &impl EntityById) -> DbResultMultiple<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let bikes = sqlx::query_as!(
            Bike,
            r#"
            UPDATE "Bike" SET
                hidden = false,
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

    pub async fn hide(&self, params: &impl EntityById) -> DbResultMultiple<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let bikes = sqlx::query_as!(
            Bike,
            r#"
            UPDATE "Bike" SET
                hidden = true,
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

    pub async fn make_clone(&self, params: &impl EntityById) -> DbResultSingle<Bike> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let bike = sqlx::query_as!(
            Bike,
            r#"
            INSERT INTO "Bike" (
                name,
                model_id,
                preview,
                description,
                year,
                price,
                frame,
                seat_tube_sizes,
                top_tube_size,
                height,
                headset,
                crankset,
                bottom_bracket,
                front_derail,
                rear_derail,
                brakes,
                shifters,
                brake_levers,
                saddle,
                seat_post,
                hubs,
                rims,
                handlebar,
                stem
            )
            SELECT
                name,
                model_id,
                NULL,
                description,
                year,
                price,
                frame,
                seat_tube_sizes,
                top_tube_size,
                height,
                headset,
                crankset,
                bottom_bracket,
                front_derail,
                rear_derail,
                brakes,
                shifters,
                brake_levers,
                saddle,
                seat_post,
                hubs,
                rims,
                handlebar,
                stem
            FROM "Bike"
            WHERE id=$1
            RETURNING *
            "#,
            params.id(),
        )
        .fetch_one(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(bike)
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

        let maybe_bike = sqlx::query_as::<_, BikeDetail>(
            r#"
            SELECT
                bike.id,
                bike.model_id,
                bike.name,
                bike.description,
                bike.view_count,
                bike.like_count,
                bike.created_at,
                bike.edited_at,
                bike.hidden,
                bike.year,
                bike.price,
                bike.frame,
                bike.seat_tube_sizes,
                bike.top_tube_size,
                bike.height,
                bike.headset,
                bike.crankset,
                bike.bottom_bracket,
                bike.front_derail,
                bike.rear_derail,
                bike.brakes,
                bike.shifters,
                bike.brake_levers,
                bike.saddle,
                bike.seat_post,
                bike.hubs,
                bike.rims,
                bike.handlebar,
                bike.stem,
                bike.status, 
                
                brand.id as brand_id,
                brand.name as brand_name,
                model.name as model_name,

                image.path as preview_path,
                image.width as preview_width,
                image.height as preview_height,
                image.thumbnail_path as preview_thumbnail_path
            FROM
                "Brand" AS brand
                    INNER JOIN
                "Model" AS model ON brand.id = model.brand_id
                    INNER JOIN
                "Bike" AS bike ON model.id = bike.model_id
                    LEFT JOIN
                "Image" AS image ON bike.preview = image.id
            WHERE
                bike.id = $1
            "#,
        )
        .bind(params.id)
        .fetch_optional(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        let bike = entity_is_correct(
            maybe_bike,
            EntityError::new(BikeDeleted, BikeDoesNotExist),
            params.fetch_hidden(),
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
                bike.description,
                bike.view_count,
                bike.like_count,
                bike.created_at,
                bike.edited_at,
                bike.hidden,
                bike.year,
                bike.price,
                bike.frame,
                bike.seat_tube_sizes,
                bike.top_tube_size,
                bike.height,
                bike.headset,
                bike.crankset,
                bike.bottom_bracket,
                bike.front_derail,
                bike.rear_derail,
                bike.brakes,
                bike.shifters,
                bike.brake_levers,
                bike.saddle,
                bike.seat_post,
                bike.hubs,
                bike.rims,
                bike.handlebar,
                bike.stem,
                bike.status,

                brand.id   AS brand_id,
                brand.name AS brand_name,
                model.name AS model_name,

                image.path as preview_path,
                image.width as preview_width,
                image.height as preview_height,
                image.thumbnail_path as preview_thumbnail_path
            FROM
                "Brand" AS brand
                    INNER JOIN
                "Model" AS model ON brand.id = model.brand_id
                    INNER JOIN
                "Bike" AS bike ON model.id = bike.model_id
                    LEFT JOIN
                "Image" AS image ON bike.preview = image.id
                    LEFT JOIN
                "BikeTag" AS tag ON tag.bike_id = bike.id  
            WHERE
                (bike.name = $1 OR $1 IS NULL)
                AND (brand_id = $2 OR $2 IS NULL)
                AND (bike.model_id = $3 OR $3 IS NULL)
                AND (brand.name = $4 OR $4 IS NULL)
                AND (model.name = $5 OR $5 IS NULL)
                AND (tag.tag_id = $6 OR $6 IS NULL)
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
            .bind(&params.tag_id)
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
            INSERT INTO "Bike" (
                name,
                model_id,
                preview,
                description,
                year,
                price,
                frame,
                seat_tube_sizes,
                top_tube_size,
                height,
                headset,
                crankset,
                bottom_bracket,
                front_derail,
                rear_derail,
                brakes,
                shifters,
                brake_levers,
                saddle,
                seat_post,
                hubs,
                rims,
                handlebar,
                stem
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24)
            RETURNING *
            "#,
            params.name,
            params.model_id,
            params.preview,
            params.description,
            params.year,
            params.price,
            params.frame,
            params.seat_tube_sizes,
            params.top_tube_size,
            params.height,
            params.headset,
            params.crankset,
            params.bottom_bracket,
            params.front_derail,
            params.rear_derail,
            params.brakes,
            params.shifters,
            params.brake_levers,
            params.saddle,
            params.seat_post,
            params.hubs,
            params.rims,
            params.handlebar,
            params.stem,
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
                preview = COALESCE($3, preview),
                description = COALESCE($4, description),
                view_count = COALESCE($5, view_count),
                like_count = COALESCE($6, like_count),
                hidden = COALESCE($7, hidden),
                year = COALESCE($8, year),
                price = COALESCE($9, price),
                frame = COALESCE($10, frame),
                seat_tube_sizes = COALESCE($11, seat_tube_sizes),
                top_tube_size = COALESCE($12, top_tube_size),
                height = COALESCE($13, height),
                headset = COALESCE($14, headset),
                crankset = COALESCE($15, crankset),
                bottom_bracket = COALESCE($16, bottom_bracket),
                front_derail = COALESCE($17, front_derail),
                rear_derail = COALESCE($18, rear_derail),
                brakes = COALESCE($19, brakes),
                shifters = COALESCE($20, shifters),
                brake_levers = COALESCE($21 , brake_levers),
                saddle = COALESCE($22, saddle),
                seat_post = COALESCE($23, seat_post),
                hubs = COALESCE($24, hubs),
                rims = COALESCE($25, rims),
                handlebar = COALESCE($26, handlebar),
                stem = COALESCE($27, stem),
                status = COALESCE($28, status),
                edited_at = current_timestamp
            WHERE id = $29
            RETURNING *
            "#,
            params.name,
            params.model_id,
            params.preview,
            params.description,
            params.view_count,
            params.like_count,
            params.hidden,
            params.year,
            params.price,
            params.frame,
            params.seat_tube_sizes,
            params.top_tube_size,
            params.height,
            params.headset,
            params.crankset,
            params.bottom_bracket,
            params.front_derail,
            params.rear_derail,
            params.brakes,
            params.shifters,
            params.brake_levers,
            params.saddle,
            params.seat_post,
            params.hubs,
            params.rims,
            params.handlebar,
            params.stem,
            params.status,
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
            DELETE FROM "Bike"
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
