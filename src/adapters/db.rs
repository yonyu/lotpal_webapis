use crate::{domain::models::Cn649, usecases::repository::Cn649Repository};
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgCn649Repository {
    pool: PgPool,
}

impl PgCn649Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Cn649Repository for PgCn649Repository {
    async fn get_all(&self) -> Result<Vec<Cn649>, sqlx::Error> {
        sqlx::query_as!(
            Cn649,
            "SELECT * FROM lotpal.\"Cn649\""
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Cn649>, sqlx::Error> {
        sqlx::query_as!(
            Cn649,
            "SELECT * FROM lotpal.\"Cn649\" WHERE \"SerialNumber\" = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create(&self, cn649: &Cn649) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO lotpal.\"Cn649\" (\"DrawNumber1\", \"DrawNumber2\", \"DrawNumber3\", \
             \"DrawNumber4\", \"DrawNumber5\", \"DrawNumber6\", \"DrawNumber7\", \"DrawDate\") \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            cn649.draw_number1,
            cn649.draw_number2,
            cn649.draw_number3,
            cn649.draw_number4,
            cn649.draw_number5,
            cn649.draw_number6,
            cn649.draw_number7,
            cn649.draw_date
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, id: i32, cn649: &Cn649) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE lotpal.\"Cn649\" SET \"DrawNumber1\" = $1, \"DrawNumber2\" = $2, \
             \"DrawNumber3\" = $3, \"DrawNumber4\" = $4, \"DrawNumber5\" = $5, \
             \"DrawNumber6\" = $6, \"DrawNumber7\" = $7, \"DrawDate\" = $8 \
             WHERE \"SerialNumber\" = $9",
            cn649.draw_number1,
            cn649.draw_number2,
            cn649.draw_number3,
            cn649.draw_number4,
            cn649.draw_number5,
            cn649.draw_number6,
            cn649.draw_number7,
            cn649.draw_date,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM lotpal.\"Cn649\" WHERE \"SerialNumber\" = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}