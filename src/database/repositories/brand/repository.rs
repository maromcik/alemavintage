use crate::database::common::error::BackendErrorKind::{
    BikeDeleted, BikeDoesNotExist, BrandDeleted, BrandDoesNotExist, BrandUpdateParametersEmpty,
};
use crate::database::common::error::{
    BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError,
};
use crate::database::common::utilities::entity_is_correct;
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, EntityById,
    PoolHandler,
};
use sqlx::{Postgres, Transaction};

use crate::database::models::brand::*;
use crate::database::models::GetById;

#[derive(Clone)]
pub struct BrandRepository {
    pool_handler: PoolHandler,
}

impl BrandRepository {
    pub async fn get_brand<'a>(
        params: &impl EntityById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Brand> {
        let query = sqlx::query_as!(
            Brand,
            r#"
            SELECT * FROM "Brand"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;
        entity_is_correct(
            query,
            EntityError::new(BrandDeleted, BrandDoesNotExist),
            params.fetch_deleted(),
        )
    }
}

impl DbRepository for BrandRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&self) {
        self.pool_handler.disconnect().await;
    }
}

impl<T> DbReadOne<T, Brand> for BrandRepository
where
    T: EntityById,
{
    async fn read_one(&self, params: &T) -> DbResultSingle<Brand> {
        let maybe_brand = sqlx::query_as!(
            Brand,
            r#"
            SELECT * FROM "Brand"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let brand = entity_is_correct(
            maybe_brand,
            EntityError::new(BrandDeleted, BrandDoesNotExist),
            params.fetch_deleted(),
        )?;
        Ok(brand)
    }
}

impl DbReadMany<BrandSearch, Brand> for BrandRepository {
    async fn read_many(&self, params: &BrandSearch) -> DbResultMultiple<Brand> {
        let brands = sqlx::query_as!(
            Brand,
            r#"
            SELECT * FROM "Brand"
            WHERE name = $1 OR $1 IS NULL
            ORDER BY name"#,
            params.name
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(brands)
    }
}

impl DbCreate<BrandCreate, Brand> for BrandRepository {
    /// Create a new brand with the given data
    async fn create(&self, params: &BrandCreate) -> DbResultSingle<Brand> {
        let brand = sqlx::query_as!(
            Brand,
            r#"
            INSERT INTO "Brand" (name, description)
            VALUES ($1, $2)
            RETURNING *
            "#,
            params.name,
            params.description
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(brand)
    }
}

impl DbUpdate<BrandUpdate, Brand> for BrandRepository {
    async fn update(&self, params: &BrandUpdate) -> DbResultMultiple<Brand> {
        if params.update_fields_none() {
            return Err(DbError::from(BackendError::new(BrandUpdateParametersEmpty)));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;

        let brand = BrandRepository::get_brand(
            &GetById {
                id: params.id,
                fetch_deleted: true,
            },
            &mut transaction,
        )
        .await?;

        let brands = sqlx::query_as!(
            Brand,
            r#"
            UPDATE "Brand"
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description)
            WHERE id = $3
            RETURNING *
            "#,
            params.name,
            params.description,
            brand.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;
        Ok(brands)
    }
}

impl DbDelete<GetById, Brand> for BrandRepository {
    async fn delete(&self, params: &GetById) -> DbResultMultiple<Brand> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        // Check existence
        let _ = BrandRepository::get_brand(params, &mut transaction).await?;

        let brands = sqlx::query_as!(
            Brand,
            r#"
                DELETE FROM "Brand"
                WHERE id = $1
                RETURNING *
               "#,
            params.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        Ok(brands)
    }
}
