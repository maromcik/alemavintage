use crate::database::common::error::BackendErrorKind::{TagDeleted, TagDoesNotExist};
use crate::database::common::error::{DbResultMultiple, DbResultSingle, EntityError};
use crate::database::common::utilities::entity_is_correct;
use crate::database::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbRepository, EntityById, PoolHandler,
};
use crate::database::models::tag::{Tag, TagAssign, TagCreate, TagJoin, TagSearch, TagUnassign};
use crate::database::models::GetById;
use sqlx::{Postgres, Transaction};

#[derive(Clone)]
pub struct TagRepository {
    pool_handler: PoolHandler,
}

impl TagRepository {
    pub async fn get_tag<'a>(
        params: &impl EntityById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Tag> {
        let query = sqlx::query_as!(
            Tag,
            r#"
            SELECT * FROM "Tag"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;
        entity_is_correct(
            query,
            EntityError::new(TagDeleted, TagDoesNotExist),
            params.fetch_hidden(),
        )
    }
}

impl DbRepository for TagRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&self) {
        self.pool_handler.disconnect().await;
    }
}

impl DbReadMany<TagSearch, TagJoin> for TagRepository {
    async fn read_many(&self, params: &TagSearch) -> DbResultMultiple<TagJoin> {
        let tags = sqlx::query_as!(
            TagJoin,
            r#"
            SELECT
                t.id AS id,
                t.name AS name,
                bt.bike_id AS bike_id
            FROM
                "Tag" AS t
                    INNER JOIN
                "BikeTag" as bt ON bt.tag_id = t.id
            WHERE
                (t.id = $1 OR $1 IS NULL)
                AND (t.name = $2 OR $2 IS NULL)
                AND (bt.bike_id = $3 OR $3 IS NULL)
            ORDER BY name"#,
            params.id,
            params.name,
            params.bike_id
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(tags)
    }
}

impl DbCreate<TagCreate, Tag> for TagRepository {
    /// Create a new tag with the given data
    async fn create(&self, params: &TagCreate) -> DbResultSingle<Tag> {
        let tag = sqlx::query_as!(
            Tag,
            r#"
            INSERT INTO "Tag" (name)
            VALUES ($1)
            RETURNING *
            "#,
            params.name,
        )
        .fetch_one(&self.pool_handler.pool)
        .await?;

        Ok(tag)
    }
}

impl DbDelete<GetById, Tag> for TagRepository {
    async fn delete(&self, params: &GetById) -> DbResultMultiple<Tag> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        // Check existence
        let _ = TagRepository::get_tag(params, &mut transaction).await?;

        let tags = sqlx::query_as!(
            Tag,
            r#"
                DELETE FROM "Tag"
                WHERE id = $1
                RETURNING *
               "#,
            params.id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        Ok(tags)
    }
}

impl DbCreate<TagAssign, ()> for TagRepository {
    async fn create(&self, params: &TagAssign) -> DbResultSingle<()> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        // sqlx::query_as!(
        //     Tag,
        //     r#"
        //     DELETE FROM "BikeTag"
        //     WHERE bike_id = $1
        //     "#,
        //     params.bike_id,
        // )
        //     .fetch_all(transaction.as_mut())
        //     .await?;

        for tag in &params.tags_ids {
            sqlx::query!(
                r#"
            INSERT INTO "BikeTag" (bike_id, tag_id)
            VALUES ($1, $2)
            "#,
                params.bike_id,
                tag
            )
            .fetch_one(transaction.as_mut())
            .await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}

impl DbDelete<TagUnassign, ()> for TagRepository {
    async fn delete(&self, params: &TagUnassign) -> DbResultMultiple<()> {
        let mut transaction = self.pool_handler.pool.begin().await?;

        sqlx::query!(
            r#"
                DELETE FROM "BikeTag"
                WHERE bike_id = $1 AND tag_id = $2
               "#,
            params.bike_id,
            params.tag_id
        )
        .fetch_all(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        Ok(Vec::default())
    }
}
