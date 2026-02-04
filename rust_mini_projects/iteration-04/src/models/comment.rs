use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct Comment {
    pub id: Uuid,
    pub commenter_id: Uuid,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub content: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct CommentAnonymized {
    pub id: Uuid,
    pub commenter_id: Uuid,
    pub post_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub content: String,
}

/// Structure passed to the repository for Comment creation
#[derive(Debug, Clone)]
pub struct CommentCreate {
    pub commenter_id: Uuid,
    pub post_id: Uuid,
    pub content: String,
}

impl CommentCreate {
    pub fn new(commenter_id: Uuid, post_id: Uuid, content: String) -> Self {
        Self {
            commenter_id,
            post_id,
            content,
        }
    }
}

/// Structure passed to the repository for updating a comment
#[derive(Debug, Clone)]
pub struct CommentUpdate {
    pub id: Uuid,
    pub commenter_id: Uuid,
    pub content: String,
}

impl CommentUpdate {
    pub fn new(id: Uuid, commenter_id: Uuid, content: String) -> Self {
        Self {
            id,
            commenter_id,
            content,
        }
    }
}

/// Structure passed to the repository for deleting a comment
#[derive(Debug, Clone)]
pub struct CommentDelete {
    pub id: Uuid,
    pub commenter_id: Uuid,
}

impl CommentDelete {
    pub fn new(id: Uuid, commenter_id: Uuid) -> Self {
        Self { id, commenter_id }
    }
}

/// Structure passed to the repository to get comment by ID
#[derive(Debug, Clone)]
pub struct CommentGetById {
    pub id: Uuid,
}

impl CommentGetById {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
