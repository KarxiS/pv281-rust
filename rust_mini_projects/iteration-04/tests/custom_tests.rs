#[cfg(test)]
pub mod custom_tests {
    use sqlx::PgPool;

    use social_network_databases::error::DbResultSingle;

    #[sqlx::test(fixtures("users", "posts"))]
    async fn post_read_one(pool: PgPool) -> DbResultSingle<()> {
        use social_network_databases::{
            DbPoolHandler, DbReadOne, DbRepository, PoolHandler, models::PostReadOne,
            repositories::PostRepository,
        };
        use std::sync::Arc;
        use uuid::Uuid;

        let arc_pool = Arc::new(pool);
        let mut repository = PostRepository::new(PoolHandler::new(arc_pool));

        // Case 1: Read an existing post
        // Post ID from 'tests/fixtures/posts.sql' (Post 1 by smithy.james)
        let post_id = Uuid::parse_str("8f1b3c47-410b-4d35-854f-b9b5b00a85b5").unwrap();
        let (post, user, comments) = repository
            .read_one(&PostReadOne::new(&post_id))
            .await
            .expect("Should successfully read existing post");

        // Verify Post details
        assert_eq!(post.id, post_id);
        assert_eq!(post.content, "I had the worst possible day!!! UGH.");

        // Verify User (creator) details
        assert_eq!(user.username, "smithy.james");
        assert_eq!(user.name, "James");
        assert_eq!(user.surname, "Smith");

        // Verify Comments (expecting none as per current fixtures)
        assert!(comments.is_empty());

        // Case 2: Read a non-existent post
        let non_existent_id = Uuid::nil();
        repository
            .read_one(&PostReadOne::new(&non_existent_id))
            .await
            .expect_err("Should fail for non-existent post");

        repository.disconnect().await;
        Ok(())
    }
}
