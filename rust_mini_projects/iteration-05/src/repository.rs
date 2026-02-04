use serde::{Deserialize, Serialize};
use sqlx::{Sqlite, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub is_done: bool,
}

#[derive(Clone)]
pub struct Repository {
    pool: sqlx::SqlitePool,
}

// Basic CRUD operations, nothing fancy here.
// Feel free to add more methods if you need them.
// Note: if you're going to use macros like `sqlx::query!` or `sqlx::query_as!`, the pipeline might fail.
#[allow(dead_code)]
impl Repository {
    pub async fn try_init() -> anyhow::Result<Repository> {
        let database_url =
            std::env::var("DATABASE_URL").expect("You probably forgot to create .env file");

        if !Sqlite::database_exists(&database_url).await? {
            Sqlite::create_database(&database_url).await?;
        }

        let pool = SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(1))
            .connect(&database_url)
            .await?;

        sqlx::migrate!("./db/migrations").run(&pool).await?;

        Ok(Repository { pool })
    }

    pub async fn insert(&self, text: String) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as(
            r#"
            INSERT INTO todos (text)
            VALUES ($1)
            RETURNING id, text, is_done
            "#,
        )
        .bind(text)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as(
            r#"
            SELECT id, text, is_done
            FROM todos
            ORDER BY id DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Todo, sqlx::Error> {
        let todo = sqlx::query_as(
            r#"
            SELECT id, text, is_done
            FROM todos
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn update(&self, todo: Todo) -> Result<Todo, sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE todos
            SET text = $1, is_done = $2
            WHERE id = $3
            "#,
        )
        .bind(&todo.text)
        .bind(todo.is_done)
        .bind(todo.id)
        .execute(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM todos
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_all_done(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM todos
            WHERE is_done = 1
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_everything(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM todos
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
