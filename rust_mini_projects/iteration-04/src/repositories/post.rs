use async_trait::async_trait;
use sqlx::postgres::PgRow;
use sqlx::{Postgres, Row, Transaction};
use uuid::Uuid;

use crate::common::error::{DbResultMultiple, DbResultSingle};
use crate::common::{
    DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate, PoolHandler,
};
use crate::error::{BusinessLogicError, BusinessLogicErrorKind, DbError};
use crate::models::{
    CommentAnonymized, Post, PostAnonymized, PostCreate, PostDelete, PostGetById, PostReadMany,
    PostReadOne, PostUpdate, UserAnonymized, UserGetById,
};
use crate::repositories::{CommentRepository, UserRepository};

pub struct PostRepository {
    pool_handler: PoolHandler,
    pub comment_repository: CommentRepository,
}

impl PostRepository {
    /// Function which retrieves a single post by its id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the post
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(post)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub(crate) async fn get_post<'a>(
        params: PostGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Post>> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM "Post" WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;
        Ok(post)
    }

    /// Function which checks if the post is correct (existing and not deleted)
    ///
    /// # Params
    /// - `post`: optional post retrieved from the database
    /// - `modifier_id`: option - id of the user attempting to modify the post, not checked if omitted
    ///
    /// # Returns
    /// - `Ok(post)`: when the post exists and is not deleted, (and the user can modify the post)
    /// - `Err(DbError)`: with appropriate error description otherwise
    pub(crate) fn is_post_correct(
        post: Option<Post>,
        modifier_id: Option<&Uuid>,
    ) -> DbResultSingle<Post> {
        match post {
            Some(p) => {
                if p.deleted_at.is_some() {
                    Err(DbError::from(BusinessLogicError::new(
                        BusinessLogicErrorKind::PostDeleted,
                    )))
                } else if let Some(id) = modifier_id {
                    if p.creator_id != *id {
                        Err(DbError::from(BusinessLogicError::new(
                            BusinessLogicErrorKind::UserNotCreatorOfPost,
                        )))
                    } else {
                        Ok(p)
                    }
                } else {
                    Ok(p)
                }
            }
            None => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::PostDoesNotExist,
            ))),
        }
    }

    /// Function which manually serializes the sqlx row result into a tuple of structures
    ///
    /// # Params
    /// - `row`: Row obtained from sqlx call
    ///
    /// # Returns
    /// - `Ok(PostAnonymized, UserAnonymized)` on successful deserialization
    /// - `Err(_)` from sqlx otherwise
    fn map_post_and_user(row: &PgRow) -> Result<(PostAnonymized, UserAnonymized), sqlx::Error> {
        let post_anonymized = PostAnonymized {
            id: row.try_get("post_id")?,
            created_at: row.try_get("post_created_at")?,
            edited_at: row.try_get("post_edited_at")?,
            content: row.try_get("post_content")?,
        };

        let user_anonymized = UserAnonymized {
            username: row.try_get("user_username")?,
            name: row.try_get("user_name")?,
            surname: row.try_get("user_surname")?,
            bio: row.try_get("user_bio")?,
            profile_picture: row.try_get("user_profile_picture")?,
            created_at: row.try_get("user_created_at")?,
        };

        Ok((post_anonymized, user_anonymized))
    }

    /// Function which manually maps rows retrieved by sqlx into tuples of structures
    ///
    /// # Params
    /// - `rows`: List of rows retrieved by sqlx
    ///
    /// # Returns
    /// - `Ok(Vec<(PostAnonymized, UserAnonymized)>)` on success
    /// - `Err(DbError)` otherwise
    pub fn collect_posts_and_users_anonymized(
        rows: &[PgRow],
    ) -> DbResultMultiple<(PostAnonymized, UserAnonymized)> {
        rows.iter()
            .map(Self::map_post_and_user)
            .collect::<Result<Vec<_>, sqlx::Error>>()
            .map_err(Into::into)
    }
}

#[async_trait]
impl DbRepository for PostRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self {
            comment_repository: CommentRepository::new(PoolHandler::new(pool_handler.pool.clone())),
            pool_handler,
        }
    }

    #[inline]
    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<PostCreate, Post> for PostRepository {
    /// Create a new post if the user exists and is not deleted
    async fn create(&mut self, data: &PostCreate) -> DbResultSingle<Post> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let user_opt =
            UserRepository::get_user(UserGetById::new(&data.creator_id), &mut tx).await?;
        UserRepository::user_is_correct(user_opt)?;

        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO "Post" (creator_id, created_at, edited_at, content)
            values($1, now(),now(),$2 )
            returning *
            "#,
            data.creator_id,
            data.content,
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(post)
    }
}

#[async_trait]
impl DbReadOne<PostReadOne, (PostAnonymized, UserAnonymized, Vec<CommentAnonymized>)>
    for PostRepository
{
    async fn read_one(
        &mut self,
        params: &PostReadOne,
    ) -> DbResultSingle<(PostAnonymized, UserAnonymized, Vec<CommentAnonymized>)> {
        // 1. Fetch Post and User data (single row) using a JOIN.
        let post_user_row = sqlx::query(
            r#"
            SELECT
                p.id as post_id,
                p.created_at as post_created_at,
                p.edited_at as post_edited_at,
                p.content as post_content,
                u.username as user_username,
                u.name as user_name,
                u.surname as user_surname,
                u.bio as user_bio,
                u.profile_picture as user_profile_picture,
                u.created_at as user_created_at
            FROM "Post" p
            JOIN "User" u ON p.creator_id = u.id
            WHERE p.id = $1 AND p.deleted_at IS NULL AND u.deleted_at IS NULL
            "#,
        )
        .bind(params.id)
        .fetch_optional(self.pool_handler.pool.as_ref())
        .await?;

        let (post_anon, user_anon) = match post_user_row {
            Some(row) => Self::map_post_and_user(&row)?,
            None => {
                return Err(DbError::from(BusinessLogicError::new(
                    BusinessLogicErrorKind::PostDoesNotExist,
                )));
            }
        };

        // 2. Fetch Comments (multiple rows).
        let comments = sqlx::query_as!(
            CommentAnonymized,
            r#"
            SELECT id, commenter_id, post_id, created_at, edited_at, content FROM "Comment"
            WHERE post_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            "#,
            params.id
        )
        .fetch_all(self.pool_handler.pool.as_ref())
        .await?;

        // 3. Combine and return `Ok((post_anon, user_anon, comments))`.
        Ok((post_anon, user_anon, comments))
    }
}

#[async_trait]
impl DbReadMany<PostReadMany, (PostAnonymized, UserAnonymized)> for PostRepository {
    /// Get existing, non deleted posts, along with their creators. "Anonymize" the data.
    /// Implement optional query parameters:
    /// - `order_by_created_at`: if not present, order in descending order
    /// - `order_by_username`: if not present, order in ascending order
    /// - `limit` and `offset`: if limit is not present, offset does not matter, if it is, use it to
    ///                         limit the number of posts retrieved. If offset is not present, use
    ///                         `OFFSET 0` as the default value
    async fn read_many(
        &mut self,
        params: &PostReadMany,
    ) -> DbResultMultiple<(PostAnonymized, UserAnonymized)> {
        use crate::common::query_parameters::DbOrder;

        let created_at_order = params
            .order_by_created_at
            .as_ref()
            .unwrap_or(&DbOrder::Desc);
        let username_order = params.order_by_username.as_ref().unwrap_or(&DbOrder::Asc);

        let query_str = format!(
            r#"
            SELECT
                p.id as post_id,
                p.created_at as post_created_at,
                p.edited_at as post_edited_at,
                p.content as post_content,
                u.username as user_username,
                u.name as user_name,
                u.surname as user_surname,
                u.bio as user_bio,
                u.profile_picture as user_profile_picture,
                u.created_at as user_created_at
            FROM "Post" p
            JOIN "User" u ON p.creator_id = u.id
            WHERE p.deleted_at IS NULL AND u.deleted_at IS NULL
            ORDER BY p.created_at {}, u.username {}
            LIMIT $1 OFFSET $2
            "#,
            created_at_order, username_order
        ); //used this way, bcs i couldnt add parameters into sqlx

        let limit = params.limit.unwrap_or(i64::MAX);
        let offset = params.offset.unwrap_or(0);

        let rows = sqlx::query(&query_str)
            .bind(limit)
            .bind(offset)
            .fetch_all(self.pool_handler.pool.as_ref())
            .await?;

        Self::collect_posts_and_users_anonymized(&rows)
    }
}

#[async_trait]
impl DbUpdate<PostUpdate, Post> for PostRepository {
    /// Update the post if the post exists, is not deleted, if the user exists, is not deleted, and
    /// is the author of the post. Hint: Use the defined checking functions in the repository
    /// definitions (both User and Post)
    async fn update(&mut self, params: &PostUpdate) -> DbResultMultiple<Post> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let user_opt =
            UserRepository::get_user(UserGetById::new(&params.creator_id), &mut tx).await?;
        UserRepository::user_is_correct(user_opt)?;

        let post = Self::get_post(PostGetById::new(&params.id), &mut tx).await?;
        Self::is_post_correct(post, Some(&params.creator_id))?;

        let updated_post = sqlx::query_as!(
            Post,
            r#"
            UPDATE "Post"
            SET content = $1, edited_at = now()
            WHERE id = $2
            RETURNING *
            "#,
            params.content,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(updated_post)
    }
}

#[async_trait]
impl DbDelete<PostDelete, Post> for PostRepository {
    /// Delete the post if the post exists, is not deleted, if the user exists, is not deleted, and
    /// is the author of the post. Hint: Use the defined checking functions in the repository
    /// definitions (both User and Post)
    async fn delete(&mut self, params: &PostDelete) -> DbResultMultiple<Post> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let user_opt =
            UserRepository::get_user(UserGetById::new(&params.creator_id), &mut tx).await?;
        UserRepository::user_is_correct(user_opt)?;

        let post = Self::get_post(PostGetById::new(&params.id), &mut tx).await?;
        Self::is_post_correct(post, Some(&params.creator_id))?;

        // Soft delete comments
        sqlx::query!(
            r#"
            UPDATE "Comment"
            SET deleted_at = now()
            WHERE post_id = $1
            "#,
            params.id
        )
        .execute(&mut *tx)
        .await?;

        // Soft delete post
        let deleted_post = sqlx::query_as!(
            Post,
            r#"
            UPDATE "Post"
            SET deleted_at = now(), edited_at = now()
            WHERE id = $1
            RETURNING *
            "#,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(deleted_post)
    }
}
