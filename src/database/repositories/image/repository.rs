use crate::database::common::error::BackendErrorKind::{BikeDeleted, BikeDoesNotExist};
use crate::database::common::error::{DbResultMultiple, DbResultSingle, EntityError};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, EntityById, PoolHandler,
};

use crate::database::common::utilities::{entity_is_correct, generate_query_param_string};
use crate::database::models::image::{
    BikeImage, BikeImageGetById, BikeImageSearch, BikeImagesCreate, Image, ImageCreate, OtherImage
    , OtherImageSearch, OtherImageType, OtherImagesCreate,
};
use crate::database::models::GetById;

#[derive(Clone)]
pub struct ImageRepository {
    pool_handler: PoolHandler,
}

impl DbRepository for ImageRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&self) {
        self.pool_handler.disconnect().await;
    }
}

impl DbReadMany<BikeImageSearch, BikeImage> for ImageRepository {
    async fn read_many(&self, params: &BikeImageSearch) -> DbResultMultiple<BikeImage> {
        let mut query = r#"
            SELECT
                image.id,
                bike_image.bike_id,
                image.path,
                image.width,
                image.height,
                image.thumbnail_path
            FROM
                "Image" AS image 
                    INNER JOIN
                 "BikeImage" as bike_image ON bike_image.image_id = image.id
            WHERE
                bike_image.bike_id = $1 OR $1 IS NULL
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

impl DbReadMany<OtherImageSearch, OtherImage> for ImageRepository {
    async fn read_many(&self, params: &OtherImageSearch) -> DbResultMultiple<OtherImage> {
        let images = sqlx::query_as!(
            OtherImage,
            r#"
            SELECT
                image.id,
                image.path,
                image.width,
                image.height,
                image.thumbnail_path,
                other_image_type.id AS image_type,
                other_image_type.name AS image_type_name
            FROM
                "Image" AS image 
                    INNER JOIN
                 "OtherImage" AS other_image ON other_image.image_id = image.id
                    INNER JOIN
                 "OtherImageType" AS other_image_type ON other_image.image_type = other_image_type.id
            WHERE
                other_image.image_type = $1 OR $1 IS NULL
            "#,
            params.image_type
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;

        Ok(images)
    }
}

impl DbCreate<ImageCreate, Image> for ImageRepository {
    async fn create(&self, data: &ImageCreate) -> DbResultSingle<Image> {
        let bike_image = sqlx::query_as!(
            Image,
            r#"
                    INSERT INTO "Image" (path, width, height, thumbnail_path)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                "#,
            data.path,
            data.width,
            data.height,
            data.thumbnail_path,
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;
        Ok(bike_image)
    }
}

impl DbCreate<BikeImagesCreate, Vec<Image>> for ImageRepository {
    async fn create(&self, data: &BikeImagesCreate) -> DbResultSingle<Vec<Image>> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let mut images = Vec::default();

        for image in &data.bike_images {
            let bike_image = sqlx::query_as!(
                Image,
                r#"
                    INSERT INTO "Image" (path, width, height, thumbnail_path)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                "#,
                image.path,
                image.width,
                image.height,
                image.thumbnail_path,
            )
            .fetch_one(transaction.as_mut())
            .await?;

            sqlx::query!(
                r#"
                    INSERT INTO "BikeImage" (bike_id, image_id)
                    VALUES ($1, $2)
                "#,
                data.bike_id,
                bike_image.id,
            )
            .execute(transaction.as_mut())
            .await?;
            images.push(bike_image);
        }
        transaction.commit().await?;
        Ok(images)
    }
}

impl DbCreate<OtherImagesCreate, Vec<Image>> for ImageRepository {
    async fn create(&self, data: &OtherImagesCreate) -> DbResultSingle<Vec<Image>> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let mut images = Vec::default();
        for image in &data.images {
            let other_image = sqlx::query_as!(
                Image,
                r#"
                    INSERT INTO "Image" (path, width, height, thumbnail_path)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                "#,
                image.path,
                image.width,
                image.height,
                image.thumbnail_path,
            )
            .fetch_one(transaction.as_mut())
            .await?;

            sqlx::query!(
                r#"
                    INSERT INTO "OtherImage" (
                        image_type,
                        image_id
                    )
                    VALUES ($1, $2)
                "#,
                data.image_type,
                other_image.id,
            )
            .execute(transaction.as_mut())
            .await?;
            images.push(other_image);
        }

        transaction.commit().await?;
        Ok(images)
    }
}

impl DbDelete<BikeImageGetById, Image> for ImageRepository {
    async fn delete(&self, params: &BikeImageGetById) -> DbResultMultiple<Image> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let images = sqlx::query_as!(
            Image,
            r#"
                DELETE 
                FROM "Image" AS image 
                USING "BikeImage" AS bike_image 
                WHERE image.id = bike_image.image_id AND bike_image.bike_id = $1
                RETURNING id, path, width, height, thumbnail_path
            "#,
            params.id(),
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(images)
    }
}

impl DbDelete<GetById, Image> for ImageRepository {
    async fn delete(&self, params: &GetById) -> DbResultMultiple<Image> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let images = sqlx::query_as!(
            Image,
            r#"
                DELETE FROM "Image"
                WHERE id = $1
                RETURNING *
            "#,
            params.id(),
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(images)
    }
}

impl<T> DbReadOne<T, BikeImage> for ImageRepository
where
    T: EntityById,
{
    async fn read_one(&self, params: &T) -> DbResultSingle<BikeImage> {
        let maybe_image = sqlx::query_as!(
            BikeImage,
            r#"
            SELECT 
                image.id,
                bike_image.bike_id,
                image.path,
                image.width,
                image.height,
                image.thumbnail_path
            FROM "Image" AS image LEFT JOIN "BikeImage" AS bike_image ON image.id = bike_image.image_id 
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;
        entity_is_correct(
            maybe_image,
            EntityError::new(BikeDeleted, BikeDoesNotExist),
            params.fetch_hidden(),
        )
    }
}

impl DbReadMany<(), OtherImageType> for ImageRepository {
    async fn read_many(&self, _params: &()) -> DbResultMultiple<OtherImageType> {
        let types = sqlx::query_as!(
            OtherImageType,
            r#"
            SELECT * FROM "OtherImageType"
            ORDER BY name"#,
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(types)
    }
}
