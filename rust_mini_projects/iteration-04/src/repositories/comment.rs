use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::common::error::{DbResultMultiple, DbResultSingle};
use crate::common::{DbCreate, DbDelete, DbPoolHandler, DbRepository, DbUpdate, PoolHandler};
use crate::error::{BusinessLogicError, BusinessLogicErrorKind, DbError};
use crate::models::{Comment, CommentCreate, CommentDelete, CommentGetById, CommentUpdate};

pub struct CommentRepository {
    pool_handler: PoolHandler,
}

impl CommentRepository {
    /// Helper to get a comment by ID within a transaction
    pub async fn get_comment<'a>(
        params: CommentGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Comment>> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            SELECT * FROM "Comment" WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;
        Ok(comment)
    }

    /// Helper to check if a comment exists, is not deleted, and optionally checks ownership
    pub fn is_comment_correct(
        comment: Option<Comment>,
        modifier_id: Option<&Uuid>,
    ) -> DbResultSingle<Comment> {
        match comment {
            Some(c) => {
                if c.deleted_at.is_some() {
                    return Err(DbError::from(BusinessLogicError::new(
                        BusinessLogicErrorKind::CommentDeleted,
                    )));
                }
                if let Some(uid) = modifier_id
                    && c.commenter_id != *uid
                {
                    return Err(DbError::from(BusinessLogicError::new(
                        BusinessLogicErrorKind::UserNotCreatorOfComment,
                    )));
                }
                Ok(c)
            }
            None => Err(DbError::from(BusinessLogicError::new(
                BusinessLogicErrorKind::CommentDoesNotExist,
            ))),
        }
    }
}

#[async_trait]
impl DbRepository for CommentRepository {
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<CommentCreate, Comment> for CommentRepository {
    async fn create(&mut self, data: &CommentCreate) -> DbResultSingle<Comment> {
        let comment = sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO "Comment" (commenter_id, post_id, content, created_at, edited_at)
            VALUES ($1, $2, $3, now(), now())
            RETURNING *
            "#,
            data.commenter_id,
            data.post_id,
            data.content
        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;

        Ok(comment)
    }
}

#[async_trait]
impl DbUpdate<CommentUpdate, Comment> for CommentRepository {
    async fn update(&mut self, params: &CommentUpdate) -> DbResultMultiple<Comment> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let comment = Self::get_comment(CommentGetById::new(params.id), &mut tx).await?;
        Self::is_comment_correct(comment, Some(&params.commenter_id))?;

        let updated_comment = sqlx::query_as!(
            Comment,
            r#"
            UPDATE "Comment"
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
        Ok(updated_comment)
    }
}

#[async_trait]
impl DbDelete<CommentDelete, Comment> for CommentRepository {
    async fn delete(&mut self, params: &CommentDelete) -> DbResultMultiple<Comment> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let comment = Self::get_comment(CommentGetById::new(params.id), &mut tx).await?;
        Self::is_comment_correct(comment, Some(&params.commenter_id))?;

        let deleted_comment = sqlx::query_as!(
            Comment,
            r#"
            UPDATE "Comment"
            SET deleted_at = now()
            WHERE id = $1
            RETURNING *
            "#,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(deleted_comment)
    }
}
