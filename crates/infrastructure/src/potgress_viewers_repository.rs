use std::net::Ipv4Addr;

use domain::viewers::Viewer;
use domain::viewers_repository::ViewersRepository;
use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

pub struct PostgresViewersRepository {
    pool: PgPool,
}

#[derive(Error, Debug)]
pub enum PostgresViewersRepositoryErrors {
    #[error("No ip / id provided, don't know what to delete")]
    KeyNotProvided,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl PostgresViewersRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ViewersRepository for PostgresViewersRepository {
    type Error = PostgresViewersRepositoryErrors;

    async fn create(&self, viewer: Viewer) -> Result<Viewer, Self::Error> {
        let id = viewer.id();
        let ip = viewer.ip().to_string();
        let visit_date = viewer.visit_date();

        sqlx::query!(
            "INSERT INTO viewers (id, ip, visit_date) VALUES ($1, $2, $3)",
            id,
            ip,
            visit_date
        )
        .execute(&self.pool)
        .await?;

        Ok(viewer)
    }

    async fn get(&self, ip: Ipv4Addr) -> Result<Option<Viewer>, Self::Error> {
        let ip = ip.to_string();

        let row = sqlx::query!(
            "SELECT id, ip, visit_date FROM viewers WHERE ip = $1 LIMIT 1",
            ip
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| {
            let ip = row
                .ip
                .parse()
                .expect("ip column always holds a value written via Ipv4Addr::to_string");
            Viewer::new(row.id, ip, row.visit_date)
        }))
    }

    async fn delete(
        &self,
        id: Option<Uuid>,
        ip: Option<Ipv4Addr>,
    ) -> Result<Option<Viewer>, Self::Error> {
        if id.is_none() && ip.is_none() {
            return Err(PostgresViewersRepositoryErrors::KeyNotProvided);
        }

        let ip_text = ip.map(|ip| ip.to_string());

        let row = sqlx::query!(
            "DELETE FROM viewers
             WHERE ($1::uuid IS NOT NULL AND id = $1) OR ($2::text IS NOT NULL AND ip = $2)
             RETURNING id, ip, visit_date",
            id,
            ip_text
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| {
            let ip = row
                .ip
                .parse()
                .expect("ip column always holds a value written via Ipv4Addr::to_string");
            Viewer::new(row.id, ip, row.visit_date)
        }))
    }
}
