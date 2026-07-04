use domain::key_value_repository::KeyValueRepository;
use sqlx::PgPool;
use thiserror::Error;

pub struct PostgresKeyValueRepository {
    pool: PgPool,
}

#[derive(Error, Debug)]
pub enum PostgresKeyValueRepositoryErrors {
    #[error("Key cannot be empty")]
    EmptyKey,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl PostgresKeyValueRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl KeyValueRepository for PostgresKeyValueRepository {
    type Error = PostgresKeyValueRepositoryErrors;

    async fn get(&self, key: &str) -> Result<Option<String>, Self::Error> {
        if key.is_empty() {
            return Err(PostgresKeyValueRepositoryErrors::EmptyKey);
        }

        let row = sqlx::query!("SELECT value FROM key_value WHERE key = $1", key)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| row.value))
    }

    async fn update(&self, key: &str, value: &str) -> Result<(), Self::Error> {
        if key.is_empty() {
            return Err(PostgresKeyValueRepositoryErrors::EmptyKey);
        }

        sqlx::query!(
            "INSERT INTO key_value (key, value) VALUES ($1, $2)
             ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value",
            key,
            value
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        if key.is_empty() {
            return Err(PostgresKeyValueRepositoryErrors::EmptyKey);
        }

        sqlx::query!("DELETE FROM key_value WHERE key = $1", key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
