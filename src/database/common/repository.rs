use std::env;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::database::common::error::{DbResultMultiple, DbResultSingle};
use crate::database::models::Id;

pub trait DbCreate<Create, Data> {
    /// Generic call which creates a record in the database
    ///
    /// # Arguments
    ///
    /// - `self`: mutable reference to the repository to access the pool handler
    /// - `data`: the structure which passes all the data that is necessary for creation of the
    ///         record in the database
    ///
    /// # Returns
    ///
    /// - `Ok(Data)` on success (the provided structure which represents
    ///                          data coming from the database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn create(&self, data: &Create) -> DbResultSingle<Data>;
}

pub trait DbReadOne<ReadOne, Data> {
    /// Generic call which reads a single record from the database
    ///
    /// # Arguments
    ///
    /// - `self`: mutable reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the read operation
    ///
    /// # Returns
    ///
    /// - `Ok(Data)` on success (the provided structure which represents read data coming
    ///                          from the database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn read_one(&self, params: &ReadOne) -> DbResultSingle<Data>;
}

pub trait DbReadMany<ReadMany, Data> {
    /// Generic call which reads multiple records from the database
    ///
    /// # Arguments
    ///
    /// - `self`: mutable reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the read operation
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Data>)` on success (a vector of structures which represent read data from the
    ///                               database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn read_many(&self, params: &ReadMany) -> DbResultMultiple<Data>;
}

pub trait DbUpdate<Update, Data> {
    /// Generic call which updates record(s) present in the database
    ///
    /// # Arguments
    ///
    /// - `self`: mutable reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the update operation
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Data>)` on success (a vector of structures which represent updated data from the
    ///                               database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn update(&self, params: &Update) -> DbResultMultiple<Data>;
}

pub trait DbDelete<Delete, Data> {
    /// Generic call which deletes record(s) present in the database
    ///
    /// # Arguments
    ///
    /// - `self`: mutable reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the delete operation
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Data>)` on success (a vector of structures which represent deleted data from the
    ///                               database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn delete(&self, params: &Delete) -> DbResultMultiple<Data>;
}

pub trait DbPoolHandler {
    /// Pool handler constructor
    #[must_use]
    fn new(pool: PgPool) -> Self;

    /// Method which allows the pool handler to disconnect from the pool
    async fn disconnect(&self) -> ();
}

/// Generic Postgres pool handler for repositories
/// (implemented to reduce code repetition)
#[derive(Clone)]
pub struct PoolHandler {
    pub(crate) pool: PgPool,
}

impl DbPoolHandler for PoolHandler {
    /// Database pool constructor
    #[must_use]
    fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Method allowing the database pool handler to disconnect from the database pool gracefully
    async fn disconnect(&self) -> () {
        self.pool.close().await;
    }
}

/// Database repository trait - implements a constructor, optionally implements any of the traits
/// that are defined in this file.

pub trait DbRepository {
    /// Database repository constructor
    #[must_use]
    fn new(pool_handler: PoolHandler) -> Self;

    /// Method allowing the database repository to disconnect from the database pool gracefully
    #[allow(dead_code)]
    async fn disconnect(&self) -> ();
}

pub async fn setup_pool(max_conn: u32) -> anyhow::Result<PgPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&database_url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

pub trait EntityById {
    fn id(&self) -> Id;
    fn fetch_deleted(&self) -> bool;
}
