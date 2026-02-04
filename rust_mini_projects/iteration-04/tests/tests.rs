#[cfg(test)]
pub mod user_repo_tests {
    use std::sync::Arc;

    use chrono::{DateTime, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use social_network_databases::error::DbResultSingle;
    use social_network_databases::models::{
        Post, PostsByUserProtected, UserCreate, UserDelete, UserLogin, UserUpdate,
    };
    use social_network_databases::repositories::{
        UserRepository, UserRepositoryListPostsProtected,
    };
    use social_network_databases::{
        DbCreate, DbDelete, DbPoolHandler, DbReadOne, DbRepository, DbUpdate, PoolHandler,
    };

    #[sqlx::test(fixtures("users"))]
    async fn create(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool));

        let user = user_repository
            .create(&UserCreate::new(
                "thomass702",
                "492999@mail.muni.cz",
                "Tomas",
                "Sedlacek",
                "There is no bio, go home!",
                "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQzyyyM6oSKhwdprn-Q4oNPfDrDr0aLdcA6JQ&s",
                "aeb729832dc9188df81cd32a948754fc73decefb",
                "wt{B#(8&{54Sbvx386SSZSN05nQ0/+Wt",
            ))
            .await
            .expect("The repository call should succeed");

        let time = Utc::now();
        let time_difference_created = time - user.created_at;
        let time_difference_edited = time - user.edited_at;

        assert!(time_difference_created.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);
        assert!(user.deleted_at.is_none());
        assert_eq!(user.username, "thomass702");
        assert_eq!(user.email, "492999@mail.muni.cz");
        assert_eq!(user.name, "Tomas");
        assert_eq!(user.surname, "Sedlacek");
        assert_eq!(user.bio, "There is no bio, go home!");
        assert_eq!(
            user.profile_picture,
            "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQzyyyM6oSKhwdprn-Q4oNPfDrDr0aLdcA6JQ&s"
        );
        assert_eq!(
            user.password_hash,
            "aeb729832dc9188df81cd32a948754fc73decefb"
        );
        assert_eq!(user.password_salt, "wt{B#(8&{54Sbvx386SSZSN05nQ0/+Wt");
        println!("User's UUID: {}", user.id);

        user_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn login(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool));

        let correct = user_repository
            .read_one(&UserLogin::new(
                "james.smith@gmail.com",
                "a5d8df21456339639232b80485feff6ad64e5165",
            ))
            .await
            .expect("The repository call should succeed");

        assert_eq!(
            correct.id,
            Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d316").unwrap()
        );

        let nonexistent = user_repository
            .read_one(&UserLogin::new(
                "idonotexist@gmail.com",
                "a191ac9d4eb55c66a7be18d785853d7214b1edc3",
            ))
            .await
            .expect_err("The database should return an error - user never existed");

        assert_eq!(
            nonexistent.to_string(),
            "[Database Error] Business logic error: The provided email and password combination is incorrect."
        );

        let password_not_matching = user_repository
            .read_one(&UserLogin::new(
                "james.smith@gmail.com",
                "1d61c68115cbaa4e063f201d2862bad23ac18025",
            ))
            .await
            .expect_err("The database should return an error - password and email combination does not match");

        assert_eq!(
            password_not_matching.to_string(),
            "[Database Error] Business logic error: The provided email and password combination is incorrect."
        );

        user_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn update(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool));

        let user = user_repository
            .read_one(&UserLogin::new(
                "james.smith@gmail.com",
                "a5d8df21456339639232b80485feff6ad64e5165",
            ))
            .await?;

        // correct
        let correct = user_repository
            .update(&UserUpdate::new(
                &user.id,
                Some("edgelord.james"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ))
            .await
            .expect("The repository call should succeed");

        assert_eq!(correct.len(), 1);

        let correct = &correct[0];

        let time = Utc::now();
        let time_difference_edited = time - correct.edited_at;

        assert!(time_difference_edited.num_seconds() < 2);
        assert!(correct.deleted_at.is_none());
        assert_eq!(correct.id, user.id);
        assert_eq!(correct.username, "edgelord.james");
        assert_eq!(correct.email, user.email);
        assert_eq!(correct.name, user.name);
        assert_eq!(correct.surname, user.surname);
        assert_eq!(correct.bio, user.bio);
        assert_eq!(correct.profile_picture, user.profile_picture);
        assert_eq!(correct.password_hash, user.password_hash);
        assert_eq!(user.password_salt, user.password_salt);

        // non correct arguments
        let incorrect_arguments = user_repository
            .update(&UserUpdate::new(
                &user.id, None, None, None, None, None, None, None, None,
            ))
            .await
            .expect_err("The repository call should return an error - wrong update parameters (all fields are `None`)");

        assert_eq!(
            incorrect_arguments.to_string(),
            concat!(
                "[Database Error] Business logic error:",
                " The provided parameters for User update query are incorrect",
                " (no User field would be changed).",
            )
        );

        // non-existent
        let nonexistent = user_repository
            .update(&UserUpdate::new(
                &Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d222").unwrap(),
                Some("thaman69"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ))
            .await
            .expect_err("The repository call should return an error - user never existed");

        assert_eq!(
            nonexistent.to_string(),
            "[Database Error] Business logic error: The specified user does not exist!"
        );

        // deleted
        let deleted = user_repository
            .update(&UserUpdate::new(
                &Uuid::parse_str("a5401e74-a8a4-4ba1-9aa0-f9023a2d2f2f").unwrap(),
                Some("thaman69"),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ))
            .await
            .expect_err("The repository call should return an error - user has been deleted");

        assert_eq!(
            deleted.to_string(),
            "[Database Error] Business logic error: The specified user has been deleted!"
        );

        user_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users"))]
    async fn delete(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool));

        let login = user_repository
            .read_one(&UserLogin::new(
                "james.smith@gmail.com",
                "a5d8df21456339639232b80485feff6ad64e5165",
            ))
            .await
            .expect("The repository calll should succeed");

        // correct
        let correct = user_repository
            .delete(&UserDelete::new(&login.id))
            .await
            .expect("The repository call should succeed");

        assert_eq!(correct.len(), 1);

        let correct = &correct[0];

        assert_eq!(correct.id.to_string(), correct.email);
        assert_eq!(correct.id.to_string(), correct.username);

        let time = Utc::now();
        let time_difference_edited = time - correct.edited_at;
        let time_difference_deleted = time
            - correct
                .deleted_at
                .expect("The deleted_at should be initialized!");

        assert!(time_difference_deleted.num_seconds() < 2);
        assert!(time_difference_edited.num_seconds() < 2);
        assert_eq!(
            correct.edited_at,
            correct
                .deleted_at
                .expect("The deleted_at should be initialized!")
        );

        user_repository
            .read_one(&UserLogin::new(
                "james.smith@gmail.com",
                "a5d8df21456339639232b80485feff6ad64e5165",
            ))
            .await.expect_err("The database repository should return an error - the user has been deleted, cannot login as them");

        // non-existent
        let nonexistent = user_repository
            .delete(&UserDelete::new(
                &Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d222").unwrap(),
            ))
            .await
            .expect_err(
                "The database repository should return an error - deleting non-existent user",
            );

        assert_eq!(
            nonexistent.to_string(),
            "[Database Error] Business logic error: The specified user does not exist!"
        );

        // deleted
        let deleted = user_repository
            .delete(&UserDelete::new(
                &Uuid::parse_str("a5401e74-a8a4-4ba1-9aa0-f9023a2d2f2f").unwrap(),
            ))
            .await
            .expect_err("The repository call should return an error - user has been deleted");

        assert_eq!(
            deleted.to_string(),
            "[Database Error] Business logic error: The specified user has been deleted!"
        );

        user_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users", "posts"))]
    async fn list_posts_protected(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repository = UserRepository::new(PoolHandler::new(arc_pool));

        // correct
        let correct = user_repository
            .list_posts_protected(PostsByUserProtected::new(
                &Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d316").unwrap(),
            ))
            .await
            .expect("The repository call should succeed!");

        // there are 3 non deleted posts by our user
        assert_eq!(correct.len(), 3);
        assert!(correct.iter().eq(vec![
            &Post {
                id: Uuid::parse_str("11474106-ae9f-451d-9aba-b87f581bf498").unwrap(),
                creator_id: Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d316").unwrap(),
                created_at: DateTime::parse_from_rfc3339("2023-10-20 15:37:01+01:00")
                    .unwrap()
                    .into(),
                edited_at: DateTime::parse_from_rfc3339("2023-10-20 15:38:20+01:00")
                    .unwrap()
                    .into(),
                deleted_at: None,
                content: "Thankfully, the insurance company will help me...".to_owned(),
            },
            &Post {
                id: Uuid::parse_str("f53457cf-6d93-4de6-bbb3-730f28968d59").unwrap(),
                creator_id: Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d316").unwrap(),
                created_at: DateTime::parse_from_rfc3339("2023-10-20 15:31:25+01:00")
                    .unwrap()
                    .into(),
                edited_at: DateTime::parse_from_rfc3339("2023-10-20 15:31:25+01:00")
                    .unwrap()
                    .into(),
                deleted_at: None,
                content: "As if this day could not get any worse... My car has broken".to_owned(),
            },
            &Post {
                id: Uuid::parse_str("8f1b3c47-410b-4d35-854f-b9b5b00a85b5").unwrap(),
                creator_id: Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d316").unwrap(),
                created_at: DateTime::parse_from_rfc3339("2023-10-20 13:49:12+01:00")
                    .unwrap()
                    .into(),
                edited_at: DateTime::parse_from_rfc3339("2023-10-20 13:49:12+01:00")
                    .unwrap()
                    .into(),
                deleted_at: None,
                content: "I had the worst possible day!!! UGH.".to_owned(),
            },
        ]));

        // non-existent
        let nonexistent = user_repository
            .delete(&UserDelete::new(
                &Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d222").unwrap(),
            ))
            .await
            .expect_err(
                "The database repository should return an error - deleting non-existent user",
            );

        assert_eq!(
            nonexistent.to_string(),
            "[Database Error] Business logic error: The specified user does not exist!"
        );

        // deleted
        let deleted = user_repository
            .delete(&UserDelete::new(
                &Uuid::parse_str("a5401e74-a8a4-4ba1-9aa0-f9023a2d2f2f").unwrap(),
            ))
            .await
            .expect_err("The repository call should return an error - user has been deleted");

        assert_eq!(
            deleted.to_string(),
            "[Database Error] Business logic error: The specified user has been deleted!"
        );

        user_repository.disconnect().await;
        Ok(())
    }
}

#[cfg(test)]
pub mod post_repo_tests {
    use std::sync::Arc;

    use chrono::{DateTime, Utc};
    use sqlx::PgPool;
    use uuid::Uuid;

    use social_network_databases::error::DbResultSingle;
    use social_network_databases::models::{
        PostAnonymized, PostCreate, PostDelete, PostReadMany, PostUpdate, UserAnonymized,
    };
    use social_network_databases::query_parameters::DbOrder;
    use social_network_databases::repositories::PostRepository;
    use social_network_databases::{
        DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbRepository, DbUpdate, PoolHandler,
    };

    #[sqlx::test(fixtures("users", "posts"))]
    async fn create(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut post_repository = PostRepository::new(PoolHandler::new(arc_pool));

        let uuid_correct = &Uuid::parse_str("b70f9e3d-c4dd-43a4-a319-f3ae3abc3f3d").unwrap();

        let post_to_create = PostCreate::new(uuid_correct, "This is gonna be the bomb!");

        // user exists -> everything okay
        let created = post_repository
            .create(&post_to_create)
            .await
            .expect("The repository call should succeed");

        assert_eq!(created.creator_id, *uuid_correct);
        assert!(created.deleted_at.is_none());
        let time = Utc::now();
        let created_difference = time - created.created_at;
        assert!(created_difference.num_seconds() < 2);
        println!("Post id: {}", created.id);

        // user non-existent
        let user_nonexistent = post_repository
            .create(&PostCreate::new(
                &Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d222").unwrap(),
                &post_to_create.content,
            ))
            .await
            .expect_err(
                "The database repository should return an error - non-existent user creating a post",
            );

        assert_eq!(
            user_nonexistent.to_string(),
            "[Database Error] Business logic error: The specified user does not exist!"
        );

        // user deleted
        let user_deleted = post_repository
            .create(&PostCreate::new(
                &Uuid::parse_str("a5401e74-a8a4-4ba1-9aa0-f9023a2d2f2f").unwrap(),
                &post_to_create.content,
            ))
            .await
            .expect_err(
                "The repository call should return an error - deleted user creating a post",
            );

        assert_eq!(
            user_deleted.to_string(),
            "[Database Error] Business logic error: The specified user has been deleted!"
        );

        post_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users", "posts"))]
    async fn read_many(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut post_repository = PostRepository::new(PoolHandler::new(arc_pool));

        let sophia_anonymized = UserAnonymized {
            username: "dreadfulenergy_".to_string(),
            name: "Sophia".to_string(),
            surname: "Montgomery".to_string(),
            bio: "The world is dark and ominous, so am I.".to_string(),
            profile_picture:
                "https://i.pinimg.com/736x/7a/d6/b2/7ad6b2085cbd9749db79ca8dcf0366f2.jpg"
                    .to_string(),
            created_at: DateTime::parse_from_rfc3339("2023-10-21 18:54:23+01:00")
                .unwrap()
                .into(),
        };
        let james_anonymized = UserAnonymized {
            username: "smithy.james".to_string(),
            name: "James".to_string(),
            surname: "Smith".to_string(),
            bio: "I don't know what to put here, so imagine a nice bio.".to_string(),
            profile_picture:
            "https://preview.redd.it/pvop3cwgd4031.jpg?width=1080&crop=smart&auto=webp&s=07289d2b466fd964218560ad62f48d6b2829293b"
                .to_string(),
            created_at: DateTime::parse_from_rfc3339("2023-10-19 10:23:54+01:00")
                .unwrap()
                .into(),
        };
        let pair_one = (
            PostAnonymized {
                id: Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap(),
                created_at: DateTime::parse_from_rfc3339("2023-10-21 20:01:24+01:00")
                    .unwrap()
                    .into(),
                edited_at: DateTime::parse_from_rfc3339("2023-10-21 20:01:24+01:00")
                    .unwrap()
                    .into(),
                content: "For the world is cold, and full of vultures.".to_string(),
            },
            sophia_anonymized,
        );
        let pair_two = (
            PostAnonymized {
                id: Uuid::parse_str("11474106-ae9f-451d-9aba-b87f581bf498").unwrap(),
                created_at: DateTime::parse_from_rfc3339("2023-10-20 15:37:01+01:00")
                    .unwrap()
                    .into(),
                edited_at: DateTime::parse_from_rfc3339("2023-10-20 15:38:20+01:00")
                    .unwrap()
                    .into(),
                content: "Thankfully, the insurance company will help me...".to_string(),
            },
            james_anonymized.clone(),
        );
        let pair_three = (
            PostAnonymized {
                id: Uuid::parse_str("f53457cf-6d93-4de6-bbb3-730f28968d59").unwrap(),
                created_at: DateTime::parse_from_rfc3339("2023-10-20 15:31:25+01:00")
                    .unwrap()
                    .into(),
                edited_at: DateTime::parse_from_rfc3339("2023-10-20 15:31:25+01:00")
                    .unwrap()
                    .into(),
                content: "As if this day could not get any worse... My car has broken".to_string(),
            },
            james_anonymized.clone(),
        );
        let pair_four = (
            PostAnonymized {
                id: Uuid::parse_str("8f1b3c47-410b-4d35-854f-b9b5b00a85b5").unwrap(),
                created_at: DateTime::parse_from_rfc3339("2023-10-20 13:49:12+01:00")
                    .unwrap()
                    .into(),
                edited_at: DateTime::parse_from_rfc3339("2023-10-20 13:49:12+01:00")
                    .unwrap()
                    .into(),
                content: "I had the worst possible day!!! UGH.".to_string(),
            },
            james_anonymized.clone(),
        );

        // correct with no parameters
        let no_params = post_repository
            .read_many(&PostReadMany::new(None, None, None, None))
            .await
            .expect("The repository call should succeed - no parameters given");

        assert!(
            no_params
                .iter()
                .eq(vec![&pair_one, &pair_two, &pair_three, &pair_four])
        );

        // take
        let take = post_repository
            .read_many(&PostReadMany::new(None, None, Some(2), None))
            .await
            .expect("The repository call should succeed - take parameter given");

        assert!(take.iter().eq(vec![&pair_one, &pair_two]));

        // take and offset
        let take_offset = post_repository
            .read_many(&PostReadMany::new(None, None, Some(2), Some(2)))
            .await
            .expect("The repository call should succeed - take parameter given");

        assert!(take_offset.iter().eq(vec![&pair_three, &pair_four]));

        // order by date asc
        let order_date_desc = post_repository
            .read_many(&PostReadMany::new(Some(DbOrder::Asc), None, None, None))
            .await
            .expect("The repository call should succeed - order by created_at asc parameter given");

        assert!(
            order_date_desc
                .iter()
                .eq(vec![&pair_four, &pair_three, &pair_two, &pair_one])
        );

        // date order and take with offset
        let order_date_take_offset = post_repository
            .read_many(&PostReadMany::new(
                Some(DbOrder::Asc),
                None,
                Some(2),
                Some(1),
            ))
            .await
            .expect(
                "The repository call should succeed - order by created_at asc & limit 2 offset 1",
            );

        assert!(
            order_date_take_offset
                .iter()
                .eq(vec![&pair_three, &pair_two])
        );

        post_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users", "posts"))]
    async fn update(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut post_repository = PostRepository::new(PoolHandler::new(arc_pool));

        // correct update
        let correct = post_repository
            .update(&PostUpdate::new(
                &Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap(),
                &Uuid::parse_str("b70f9e3d-c4dd-43a4-a319-f3ae3abc3f3d").unwrap(),
                "For the world is cold, dark, and full of vultures.",
            ))
            .await
            .expect("The repository call should succeed - correct update of a post");

        assert_eq!(correct.len(), 1);
        let correct = &correct[0];
        let time = Utc::now();
        let edited_time_difference = time - correct.edited_at;
        assert!(edited_time_difference.num_seconds() < 2);
        assert!(correct.deleted_at.is_none());
        assert_eq!(
            correct.content,
            "For the world is cold, dark, and full of vultures."
        );

        // bad permissions
        let bad_permissions = post_repository
            .update(&PostUpdate::new(
                &Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap(),
                &Uuid::parse_str("12e24c86-8585-4f9d-aba4-9258e7c3e180").unwrap(),
                "misterious post, booo hoo",
            ))
            .await
            .expect_err("The repository call should fail - incorrect permissions");
        assert_eq!(
            bad_permissions.to_string(),
            "[Database Error] Business logic error: The specified user cannot change the post, as they did not create it!"
        );

        // user does not exist
        let user_does_not_exist = post_repository
            .update(&PostUpdate::new(
                &Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap(),
                &Uuid::parse_str("12e24c86-8585-4f9d-aba4-9258e7c3e181").unwrap(),
                "random content",
            ))
            .await
            .expect_err("The repository call should fail - user does not exist");
        assert_eq!(
            user_does_not_exist.to_string(),
            "[Database Error] Business logic error: The specified user does not exist!"
        );

        // user has been deleted
        let user_deleted = post_repository
            .update(&PostUpdate::new(
                &Uuid::parse_str("80d4b1ee-4aa2-4e11-b47e-ccc4483bb662").unwrap(),
                &Uuid::parse_str("a5401e74-a8a4-4ba1-9aa0-f9023a2d2f2f").unwrap(),
                "random content",
            ))
            .await
            .expect_err("The repository call should fail - user does not exist");
        assert_eq!(
            user_deleted.to_string(),
            "[Database Error] Business logic error: The specified user has been deleted!"
        );

        // post does not exist
        let post_nonexistent = post_repository
            .update(&PostUpdate::new(
                &Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739431").unwrap(),
                &Uuid::parse_str("b70f9e3d-c4dd-43a4-a319-f3ae3abc3f3d").unwrap(),
                "random content",
            ))
            .await
            .expect_err("The repository call should fail - post does not exist");
        assert_eq!(
            post_nonexistent.to_string(),
            "[Database Error] Business logic error: The specified post does not exist!"
        );

        // post has been deleted
        let post_deleted = post_repository
            .update(&PostUpdate::new(
                &Uuid::parse_str("80d4b1ee-4aa2-4e11-b47e-ccc4483bb662").unwrap(),
                &Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d316").unwrap(),
                "*totally not an FBI hack*: They were very nice to me and everything went right!",
            ))
            .await
            .expect_err("The repository call should fail - post deleted");
        assert_eq!(
            post_deleted.to_string(),
            "[Database Error] Business logic error: The specified post has been deleted!"
        );

        post_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("users", "posts"))]
    async fn delete(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut post_repository = PostRepository::new(PoolHandler::new(arc_pool));

        // correct update
        let correct = post_repository
            .delete(&PostDelete::new(
                &Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap(),
                &Uuid::parse_str("b70f9e3d-c4dd-43a4-a319-f3ae3abc3f3d").unwrap(),
            ))
            .await
            .expect("The repository call should succeed - correct delete of a post");

        assert_eq!(correct.len(), 1);
        let correct = &correct[0];
        let time = Utc::now();
        let edited_time_difference = time - correct.edited_at;
        assert!(correct.deleted_at.is_some());
        let deleted_time_difference = time - correct.deleted_at.unwrap();
        assert!(edited_time_difference.num_seconds() < 2);
        assert!(deleted_time_difference.num_seconds() < 2);
        assert_eq!(edited_time_difference, deleted_time_difference);
        assert_eq!(
            correct.created_at,
            DateTime::parse_from_rfc3339("2023-10-21 20:01:24+01:00").unwrap()
        );
        assert_eq!(
            correct.id,
            Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap()
        );
        assert_eq!(
            correct.creator_id,
            Uuid::parse_str("b70f9e3d-c4dd-43a4-a319-f3ae3abc3f3d").unwrap(),
        );
        assert_eq!(
            correct.content,
            "For the world is cold, and full of vultures."
        );
        assert_eq!(
            correct.created_at,
            DateTime::parse_from_rfc3339("2023-10-21 20:01:24+01:00").unwrap()
        );
        let read_many = post_repository
            .read_many(&PostReadMany::new(None, None, None, None))
            .await
            .expect("The repository call should succeed - listing posts");

        let find_deleted_in_listed_posts: Option<(PostAnonymized, UserAnonymized)> =
            read_many.into_iter().find(|(post, _)| {
                post.id == Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap()
            });

        assert!(find_deleted_in_listed_posts.is_none());

        // bad permissions
        let bad_permissions = post_repository
            .delete(&PostDelete::new(
                &Uuid::parse_str("11474106-ae9f-451d-9aba-b87f581bf498").unwrap(),
                &Uuid::parse_str("12e24c86-8585-4f9d-aba4-9258e7c3e180").unwrap(),
            ))
            .await
            .expect_err("The repository call should fail - incorrect permissions");
        assert_eq!(
            bad_permissions.to_string(),
            "[Database Error] Business logic error: The specified user cannot change the post, as they did not create it!"
        );

        // user does not exist
        let user_does_not_exist = post_repository
            .delete(&PostDelete::new(
                &Uuid::parse_str("11474106-ae9f-451d-9aba-b87f581bf498").unwrap(),
                &Uuid::parse_str("12e24c86-8585-4f9d-aba4-9258e7c3e181").unwrap(),
            ))
            .await
            .expect_err("The repository call should fail - user does not exist");
        assert_eq!(
            user_does_not_exist.to_string(),
            "[Database Error] Business logic error: The specified user does not exist!"
        );

        // user has been deleted
        let user_deleted = post_repository
            .delete(&PostDelete::new(
                &Uuid::parse_str("11474106-ae9f-451d-9aba-b87f581bf498").unwrap(),
                &Uuid::parse_str("a5401e74-a8a4-4ba1-9aa0-f9023a2d2f2f").unwrap(),
            ))
            .await
            .expect_err("The repository call should fail - user does not exist");
        assert_eq!(
            user_deleted.to_string(),
            "[Database Error] Business logic error: The specified user has been deleted!"
        );

        // post does not exist
        let post_nonexistent = post_repository
            .delete(&PostDelete::new(
                &Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739431").unwrap(),
                &Uuid::parse_str("b70f9e3d-c4dd-43a4-a319-f3ae3abc3f3d").unwrap(),
            ))
            .await
            .expect_err("The repository call should fail - post does not exist");
        assert_eq!(
            post_nonexistent.to_string(),
            "[Database Error] Business logic error: The specified post does not exist!"
        );

        // post has been deleted
        let post_deleted = post_repository
            .delete(&PostDelete::new(
                &Uuid::parse_str("ad2e9b6b-b26e-43c9-83d0-feaeba739432").unwrap(),
                &Uuid::parse_str("fb0f354e-192a-41d2-afc1-0edeee47d316").unwrap(),
            ))
            .await
            .expect_err("The repository call should fail - post deleted");
        assert_eq!(
            post_deleted.to_string(),
            "[Database Error] Business logic error: The specified post has been deleted!"
        );

        post_repository.disconnect().await;
        Ok(())
    }
}
