use crate::database::common::error::BackendErrorKind::{
    ModelDeleted, ModelDoesNotExist, ModelUpdateParametersEmpty,
};
use crate::database::common::error::{
    BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError,
};
use crate::database::common::utilities::entity_is_correct;
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use sqlx::{Postgres, Transaction};

use crate::database::models::model::{Model, ModelCreate, ModelSearch, ModelUpdate};
use crate::database::models::GetById;

#[derive(Clone)]
pub struct ModelRepository {
    pool_handler: PoolHandler,
}

impl ModelRepository {
    pub async fn get_brand<'a>(
        params: GetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Model>> {
        let query = sqlx::query_as!(
            Model,
            r#"
            SELECT * FROM "Model"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(brand) = query {
            return Ok(Some(brand));
        }

        Err(DbError::from(BackendError::new(ModelDoesNotExist)))
    }

    pub fn brand_is_correct(brand: Option<Model>) -> DbResultSingle<Model> {
        entity_is_correct(
            brand,
            EntityError::new(ModelDeleted, ModelDoesNotExist),
            false,
        )
    }
}

impl DbRepository for ModelRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&self) {
        self.pool_handler.disconnect().await;
    }
}

impl DbReadOne<GetById, Model> for ModelRepository {
    async fn read_one(&self, params: &GetById) -> DbResultSingle<Model> {
        let maybe_brand = sqlx::query_as!(
            Model,
            r#"
            SELECT * FROM "Model"
            WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let brand = ModelRepository::brand_is_correct(maybe_brand)?;
        Ok(brand)
    }
}

impl DbReadMany<ModelSearch, Model> for ModelRepository {
    async fn read_many(&self, params: &ModelSearch) -> DbResultMultiple<Model> {
        let brands = sqlx::query_as!(
            Model,
            r#"
            SELECT * FROM "Model"
            WHERE name = $1 OR $1 IS NULL
            ORDER BY name"#,
            params.name
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(brands)
    }
}

impl DbCreate<ModelCreate, Model> for ModelRepository {
    /// Create a new brand with the given data
    async fn create(&self, params: &ModelCreate) -> DbResultSingle<Model> {
        let brand = sqlx::query_as!(
            Model,
            r#"
            INSERT INTO "Model" (name, description)
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

impl DbUpdate<ModelUpdate, Model> for ModelRepository {
    async fn update(&self, params: &ModelUpdate) -> DbResultMultiple<Model> {
        if params.update_fields_none() {
            return Err(DbError::from(BackendError::new(ModelUpdateParametersEmpty)));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;
        let brand_id = GetById::new(params.id);

        let query_brand = ModelRepository::get_brand(brand_id, &mut transaction).await?;
        let _ = ModelRepository::brand_is_correct(query_brand);

        let brands = sqlx::query_as!(
            Model,
            r#"
            UPDATE "Model"
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description)
            WHERE id = $3
            RETURNING *
            "#,
            params.name,
            params.description,
            params.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;
        Ok(brands)
    }
}

impl DbDelete<GetById, Model> for ModelRepository {
    async fn delete(&self, params: &GetById) -> DbResultMultiple<Model> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        // Check existence
        let _ =
            ModelRepository::get_brand(GetById::new(params.id), &mut transaction).await?;

        let brands = sqlx::query_as!(
            Model,
            r#"
                DELETE FROM "Model"
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
