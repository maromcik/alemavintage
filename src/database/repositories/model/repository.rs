use crate::database::common::error::BackendErrorKind::{
    ModelDeleted, ModelDoesNotExist, ModelUpdateParametersEmpty,
};
use crate::database::common::error::{
    BackendError, DbError, DbResultMultiple, DbResultSingle, EntityError,
};
use crate::database::common::utilities::{entity_is_correct, generate_query_param_string};
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, EntityById,
    PoolHandler,
};
use crate::database::models::model::{Model, ModelCreate, ModelDetail, ModelSearch, ModelUpdate};
use crate::database::models::GetById;
use sqlx::{Postgres, Transaction};

#[derive(Clone)]
pub struct ModelRepository {
    pool_handler: PoolHandler,
}

impl ModelRepository {
    pub async fn get_model<'a>(
        params: &impl EntityById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Model> {
        let query = sqlx::query_as!(
            Model,
            r#"
            SELECT * FROM "Model"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        entity_is_correct(
            query,
            EntityError::new(ModelDeleted, ModelDoesNotExist),
            params.fetch_hidden(),
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

impl<T> DbReadOne<T, ModelDetail> for ModelRepository
where
    T: EntityById,
{
    async fn read_one(&self, params: &T) -> DbResultSingle<ModelDetail> {
        let maybe_model = sqlx::query_as!(
            ModelDetail,
            r#"
            SELECT
                model.id,
                model.brand_id,
                model.name,
                model.description,

                brand.name AS brand_name,
                brand.description AS brand_description
            FROM
            "Model" as model
                INNER JOIN
            "Brand" as brand ON (model.brand_id = brand.id)
            WHERE model.id = $1
            "#,
            params.id()
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let model = entity_is_correct(
            maybe_model,
            EntityError::new(ModelDeleted, ModelDoesNotExist),
            params.fetch_hidden(),
        )?;
        Ok(model)
    }
}

impl DbReadMany<ModelSearch, ModelDetail> for ModelRepository {
    async fn read_many(&self, params: &ModelSearch) -> DbResultMultiple<ModelDetail> {
        let mut query = r#"
            SELECT
                model.id,
                model.brand_id,
                model.name,
                model.description,

                brand.name AS brand_name,
                brand.description AS brand_description
            FROM
            "Model" as model
                INNER JOIN
            "Brand" as brand ON (model.brand_id = brand.id)
            WHERE
                (model.name = $1 OR $1 IS NULL)
                AND (brand.id = $2 OR $2 IS NULL)
            "#
        .to_owned();

        let query_params = generate_query_param_string(&params.query_params);
        query.push_str(query_params.as_str());

        let models = sqlx::query_as::<_, ModelDetail>(query.as_str())
            .bind(&params.name)
            .bind(&params.brand_id)
            .fetch_all(&self.pool_handler.pool)
            .await?;

        Ok(models)
    }
}

impl DbCreate<ModelCreate, Model> for ModelRepository {
    /// Create a new model with the given data
    async fn create(&self, params: &ModelCreate) -> DbResultSingle<Model> {
        let model = sqlx::query_as!(
            Model,
            r#"
            INSERT INTO "Model" (brand_id, name, description)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            params.brand_id,
            params.name,
            params.description
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(model)
    }
}

impl DbUpdate<ModelUpdate, Model> for ModelRepository {
    async fn update(&self, params: &ModelUpdate) -> DbResultMultiple<Model> {
        if params.update_fields_none() {
            return Err(DbError::from(BackendError::new(ModelUpdateParametersEmpty)));
        }

        let mut transaction = self.pool_handler.pool.begin().await?;

        let model = ModelRepository::get_model(
            &GetById {
                id: params.id,
                fetch_deleted: true,
            },
            &mut transaction,
        )
        .await?;

        let models = sqlx::query_as!(
            Model,
            r#"
            UPDATE "Model"
            SET
                brand_id = COALESCE($1, brand_id),
                name = COALESCE($2, name),
                description = COALESCE($3, description)
            WHERE id = $4
            RETURNING *
            "#,
            params.brand_id,
            params.name,
            params.description,
            model.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;
        Ok(models)
    }
}

impl DbDelete<GetById, Model> for ModelRepository {
    async fn delete(&self, params: &GetById) -> DbResultMultiple<Model> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        // Check existence
        let _bike = ModelRepository::get_model(params, &mut transaction).await?;

        let models = sqlx::query_as!(
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

        Ok(models)
    }
}
