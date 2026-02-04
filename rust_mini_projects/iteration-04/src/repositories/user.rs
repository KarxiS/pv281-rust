use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::common::error::{DbResultMultiple, DbResultSingle};
use crate::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::error::{BusinessLogicError, BusinessLogicErrorKind, DbError};
use crate::models::{Post, UserLogin};
use crate::models::{PostsByUserProtected, User, UserCreate, UserDelete, UserGetById, UserUpdate};

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
    pub async fn get_user<'a>(
        params: UserGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM "User" WHERE id = $1

            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;
        Ok(user)
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
        match user {
            Some(u) => {
                if u.deleted_at.is_some() {
                    Err(DbError::from(BusinessLogicError::new(
                        BusinessLogicErrorKind::UserDeleted,
                    )))
                } else {
                    Ok(u)
                }
            }
            None => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::UserDoesNotExist,
            ))),
        }
    }
}

/// trait for listing posts for the User
#[async_trait]
pub trait UserRepositoryListPostsProtected {
    /// List the posts that the user can modify (is the author of them)
    async fn list_posts_protected(
        &mut self,
        params: PostsByUserProtected,
    ) -> DbResultMultiple<Post>;
}

#[async_trait]
impl UserRepositoryListPostsProtected for UserRepository {
    async fn list_posts_protected(
        &mut self,
        params: PostsByUserProtected,
    ) -> DbResultMultiple<Post> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM "Post" WHERE creator_id = $1 and deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            params.user_id
        )
        .fetch_all(self.pool_handler.pool.as_ref())
        .await?;

        Ok(posts)
    }
}

#[async_trait]
impl DbRepository for UserRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<UserCreate, User> for UserRepository {
    /// Create a new user with the specified data
    async fn create(&mut self, data: &UserCreate) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"

            INSERT INTO "User" (username, email, name, surname, bio, profile_picture, password_hash, password_salt)
            VALUES ($1, $2, $3, $4, $5, $6, $7,$8)
            RETURNING id, username, email, name, surname, bio, profile_picture, password_hash, password_salt,created_at,edited_at, deleted_at


            "#,
            data.username,
            data.email,
            data.name,
            data.surname,
            data.bio,
            data.profile_picture,
            data.password_hash,
            data.password_salt,



        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;

        Ok(user)
    }
}

#[async_trait]
impl DbReadOne<UserLogin, User> for UserRepository {
    /// Login the user with provided parameters, if the user does not exist, is deleted or the
    /// passwords don't match, return the error about combination of email/password not working
    async fn read_one(&mut self, params: &UserLogin) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * from "User"
            WHERE email = $1 AND password_hash = $2 AND deleted_at IS NULL
            "#,
            params.email,
            params.password_hash,
        )
        .fetch_optional(self.pool_handler.pool.as_ref())
        .await?;
        match user {
            Some(user) => Ok(user),
            None => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::UserPasswordDoesNotMatch,
            ))),
        }
    }
}

#[async_trait]
impl DbUpdate<UserUpdate, User> for UserRepository {
    /// Update user information if we know their id (we're logged in as that user)
    /// Fails if the relevant update fields are all none
    async fn update(&mut self, params: &UserUpdate) -> DbResultMultiple<User> {
        // 1. Check if `params.update_fields_none()` is true.
        if params.update_fields_none() {
            return Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::UserUpdateParametersEmpty,
            )));
        }

        let mut tx = self.pool_handler.pool.begin().await?;

        // 2. Check if user exists and is correct (not deleted)
        let user_opt = Self::get_user(UserGetById::new(&params.id), &mut tx).await?;
        Self::user_is_correct(user_opt)?;

        // 3. Perform the update
        let updated_users = sqlx::query_as!(
            User,
            r#"
            UPDATE "User" SET
                username = COALESCE($2, username),
                email = COALESCE($3, email),
                name = COALESCE($4, name),
                surname = COALESCE($5, surname),
                bio = COALESCE($6, bio),
                profile_picture = COALESCE($7, profile_picture),
                password_hash = COALESCE($8, password_hash),
                password_salt = COALESCE($9, password_salt),
                edited_at = now()
            WHERE id = $1
            RETURNING *
            "#,
            params.id,
            params.username,
            params.email,
            params.name,
            params.surname,
            params.bio,
            params.profile_picture,
            params.password_hash,
            params.password_salt,
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        if updated_users.is_empty() {
            return Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::UserDoesNotExist,
            )));
        }

        Ok(updated_users)
    }
}

#[async_trait]
impl DbDelete<UserDelete, User> for UserRepository {
    /// Delete the user if we know their id (we're logged in as that user)
    async fn delete(&mut self, params: &UserDelete) -> DbResultMultiple<User> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // 1. Check if user exists and is correct (not deleted)
        let user_opt = Self::get_user(UserGetById::new(&params.id), &mut tx).await?;
        Self::user_is_correct(user_opt)?;

        // 2. Soft delete comments
        sqlx::query!(
            r#"
            UPDATE "Comment" SET deleted_at = now() WHERE commenter_id = $1 AND deleted_at IS NULL
            "#,
            params.id,
        )
        .execute(&mut *tx)
        .await?;

        // 3. Soft delete posts
        sqlx::query!(
            r#"
            UPDATE "Post" SET deleted_at = now() WHERE creator_id = $1 AND deleted_at IS NULL
            "#,
            params.id,
        )
        .execute(&mut *tx)
        .await?;

        // 4. Soft delete user
        let users = sqlx::query_as!(
            User,
            r#"
            UPDATE "User"
            SET deleted_at = now(), edited_at = now(), username = $1::text, email = $1::text
            WHERE id = $1
            RETURNING *
            "#,
            params.id,
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        if users.is_empty() {
            return Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::UserDoesNotExist,
            )));
        }

        Ok(users)
    }
}
