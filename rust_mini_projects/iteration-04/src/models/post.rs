use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::query_parameters::DbOrder;

/// Post structure which is serialized from the database, containing full information about the post
/// only obtainable when listing own posts, or creating a new post.
#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Post {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub content: String,
}

/// Post structure for obtaining information about other posts (when listing posts,
/// or when showing a post detail), does not contain sensitive information about them
#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct PostAnonymized {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub content: String,
}

/// Structure passed to the repository for post creation
#[derive(Debug, Clone)]
pub struct PostCreate {
    pub creator_id: Uuid,
    pub content: String,
}

impl PostCreate {
    #[must_use]
    #[inline]
    pub fn new(creator_id: &Uuid, content: &str) -> Self {
        Self {
            creator_id: *creator_id,
            content: content.to_owned(),
        }
    }
}

/// Structure passed to the repository for getting details about one post
pub struct PostReadOne {
    pub id: Uuid,
}

impl PostReadOne {
    #[must_use]
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository for getting multiple posts, supporting pagination
#[derive(Debug, Clone)]
pub struct PostReadMany {
    pub order_by_created_at: Option<DbOrder>,
    pub order_by_username: Option<DbOrder>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl PostReadMany {
    #[inline]
    #[must_use]
    pub const fn new(
        order_by_created_at: Option<DbOrder>,
        order_by_username: Option<DbOrder>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Self {
        Self {
            order_by_created_at,
            order_by_username,
            limit,
            offset,
        }
    }
}

/// Structure passed to the repository for updating posts
#[derive(Debug, Clone)]
pub struct PostUpdate {
    pub id: Uuid,
    pub creator_id: Uuid,
    pub content: String,
}

impl PostUpdate {
    #[must_use]
    #[inline]
    pub fn new(id: &Uuid, creator_id: &Uuid, content: &str) -> Self {
        Self {
            id: *id,
            creator_id: *creator_id,
            content: content.to_owned(),
        }
    }
}

// Structure passed to the repository for deleting posts
#[derive(Debug, Clone)]
pub struct PostDelete {
    pub id: Uuid,
    pub creator_id: Uuid,
}

impl PostDelete {
    #[must_use]
    #[inline]
    pub const fn new(id: &Uuid, creator_id: &Uuid) -> Self {
        Self {
            id: *id,
            creator_id: *creator_id,
        }
    }
}

// Structure passed to the repository to retrieve a post by its id
#[derive(Debug, Clone)]
pub struct PostGetById {
    pub id: Uuid,
}

impl PostGetById {
    #[must_use]
    #[inline]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}
