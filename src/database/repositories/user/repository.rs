use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use rand_core::OsRng;

use sqlx::{Postgres, Transaction};

use crate::database::common::error::BackendErrorKind::{
    UserDeleted, UserDoesNotExist, UserPasswordDoesNotMatch, UserUpdateParametersEmpty,
};
use crate::database::common::error::{BackendError, DbError, EntityError};
use crate::database::common::error::{DbResultMultiple, DbResultSingle};
use crate::database::common::utilities::entity_is_correct;
use crate::database::common::{
    DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, EntityById, PoolHandler,
};
use crate::database::models::user::{User, UserLogin, UserSearch, UserUpdate, UserUpdatePassword};
use crate::database::models::GetById;

fn generate_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

fn hash_password(password: String, salt: &SaltString) -> Result<String, DbError> {
    let password_hash = Pbkdf2.hash_password(password.as_bytes(), salt)?.to_string();
    Ok(password_hash)
}

fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<bool, DbError> {
    let parsed_hash = PasswordHash::new(expected_password_hash)?;
    let bytes = password_candidate.bytes().collect::<Vec<u8>>();
    Ok(Pbkdf2.verify_password(&bytes, &parsed_hash).is_ok())
}

#[derive(Clone)]
pub struct UserRepository {
    pool_handler: PoolHandler,
}

impl UserRepository {
    /// Function which retrieves a user by their id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the user
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(user)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_user<'a, T>(
        params: T,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<User>>
    where
        T: EntityById,
    {
        let query = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        if let Some(user) = query {
            return Ok(Option::from(user));
        }

        Err(DbError::from(BackendError::new(UserDoesNotExist)))
    }

    /// Function which checks if the user is correct (existing and not deleted)
    ///
    /// # Params
    /// - `user`: optional user retrieved from the database
    ///
    /// # Returns
    /// - `Ok(user)`: when the user exists and is not deleted
    /// - `Err(DbError)`: with appropriate error description otherwise
    pub fn user_is_correct(user: Option<User>) -> DbResultSingle<User> {
        entity_is_correct(user, EntityError::new(UserDeleted, UserDoesNotExist), false)
    }

    pub fn verify_password(user: User, given_password: &str) -> DbResultSingle<User> {
        match verify_password_hash(&user.password_hash, given_password) {
            Ok(ret) => {
                if ret {
                    return Ok(user);
                }
                Err(DbError::from(BackendError::new(UserPasswordDoesNotMatch)))
            }
            Err(e) => Err(e),
        }
    }

    pub async fn update_password(&self, params: &UserUpdatePassword) -> DbResultSingle<User> {
        let mut transaction = self.pool_handler.pool.begin().await?;
        let user_query = UserRepository::get_user(
            GetById {
                id: params.id,
                fetch_deleted: false,
            },
            &mut transaction,
        )
        .await?;
        let user = UserRepository::user_is_correct(user_query.clone())?;
        let user = UserRepository::verify_password(user, &params.old_password)?;

        let salt = generate_salt();
        let password_hash = hash_password(params.new_password.clone(), &salt)?;

        let users = sqlx::query_as!(
            User,
            r#"
            UPDATE "User" SET
                password_hash = $1,
                password_salt = $2
            WHERE id = $3
            RETURNING *
            "#,
            password_hash,
            salt.to_string(),
            user.id,
        )
        .fetch_one(transaction.as_mut())
        .await?;

        transaction.commit().await?;

        Ok(users)
    }
}

impl DbRepository for UserRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&self) {
        self.pool_handler.disconnect().await;
    }
}

impl DbReadOne<UserLogin, User> for UserRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&self, params: &UserLogin) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE email = $1
            "#,
            params.email
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let user = UserRepository::user_is_correct(user)?;

        UserRepository::verify_password(user, &params.password)
    }
}

impl<T> DbReadOne<T, User> for UserRepository
where
    T: EntityById,
{
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&self, params: &T) -> DbResultSingle<User> {
        let maybe_user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE id = $1
            "#,
            params.id()
        )
        .fetch_optional(&self.pool_handler.pool)
        .await?;

        let user = UserRepository::user_is_correct(maybe_user)?;
        Ok(user)
    }
}

impl DbReadMany<UserSearch, User> for UserRepository {
    async fn read_many(&self, params: &UserSearch) -> DbResultMultiple<User> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User"
            WHERE
                (admin = $1 OR $1 IS NULL) 
                AND (email = $2 OR $2 IS NULL) 
                AND (name = $3 OR $3 IS NULL) 
                AND (surname = $4 OR $4 IS NULL)
            "#,
            params.admin,
            params.email,
            params.name,
            params.surname
        )
        .fetch_all(&self.pool_handler.pool)
        .await?;
        Ok(users)
    }
}

impl DbUpdate<UserUpdate, User> for UserRepository {
    /// Update user information if we know their id (we're logged in as that user)
    /// Fails if the relevant update fields are all none
    async fn update(&self, params: &UserUpdate) -> DbResultMultiple<User> {
        if params.update_fields_none() {
            return Err(DbError::from(BackendError::new(UserUpdateParametersEmpty)));
        }
        let mut transaction = self.pool_handler.pool.begin().await?;
        let user = UserRepository::get_user(GetById::new(params.id), &mut transaction).await?;
        let validated_user = UserRepository::user_is_correct(user)?;
        let (password, salt) = match &params.password {
            Some(p) => {
                let salt = generate_salt();
                let password_hash = hash_password(p.clone(), &salt)?;
                (Some(password_hash), Some(salt.to_string()))
            }
            None => (None, None),
        };
        let updated_users = sqlx::query_as!(
            User,
            r#"
            UPDATE "User"
            SET
                email = COALESCE($1, email),
                name = COALESCE($2, name),
                surname = COALESCE($3, surname),
                password_hash = COALESCE($4, password_hash),
                password_salt = COALESCE($5, password_salt),
                admin = COALESCE($6, admin)
            WHERE id = $7
            RETURNING *
            "#,
            params.email,
            params.name,
            params.surname,
            password,
            salt,
            params.admin,
            validated_user.id
        )
        .fetch_all(transaction.as_mut())
        .await?;
        transaction.commit().await?;
        Ok(updated_users)
    }
}

// #[async_trait]
// impl DbDelete<UserDelete, User> for UserRepository {
//     /// Delete the user if we know their id (we're logged in as that user)
//     async fn delete(&self, params: &UserDelete) -> DbResultMultiple<User> {
//         //find user
//         let mut transaction = self.pool_handler.pool.begin().await?;
//         let user_query =
//             UserRepository::get_user(UserGetById { id: params.id }, &mut transaction).await?;
//
//         //user does not exist
//         let _ = UserRepository::user_is_correct(user_query.clone())?;
//
//         let users = sqlx::query_as!(
//             User,
//             r#"
//             UPDATE "User" SET
//                 username = $1,
//                 email = $1,
//                 deleted_at = $2,
//                 edited_at = $2
//             WHERE id = $1
//             RETURNING *
//             "#,
//             params.id,
//             Utc::now()
//         )
//             .fetch_all(transaction.as_mut())
//             .await?;
//
//         transaction.commit().await?;
//
//         Ok(users)
//     }
// }
