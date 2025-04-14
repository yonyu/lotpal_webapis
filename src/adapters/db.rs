use crate::{domain::models::Cn649, usecases::repository::Cn649Repository};
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
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
            r#"SELECT 
                "SerialNumber" as "serial_number!",
                "DrawNumber1" as "draw_number1!",
                "DrawNumber2" as "draw_number2!",
                "DrawNumber3" as "draw_number3!",
                "DrawNumber4" as "draw_number4!",
                "DrawNumber5" as "draw_number5!",
                "DrawNumber6" as "draw_number6!",
                "DrawNumber7" as "draw_number7!",
                "DrawDate" as "draw_date!"
            FROM lotpal."Cn649""#
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn get_by_id(&self, id: i32) -> Result<Option<Cn649>, sqlx::Error> {
        sqlx::query_as!(
            Cn649,
            r#"SELECT 
                "SerialNumber" as "serial_number!",
                "DrawNumber1" as "draw_number1!",
                "DrawNumber2" as "draw_number2!",
                "DrawNumber3" as "draw_number3!",
                "DrawNumber4" as "draw_number4!",
                "DrawNumber5" as "draw_number5!",
                "DrawNumber6" as "draw_number6!",
                "DrawNumber7" as "draw_number7!",
                "DrawDate" as "draw_date!"
            FROM lotpal."Cn649" 
            WHERE "SerialNumber" = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create(&self, cn649: &Cn649) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO lotpal."Cn649" (
                "DrawNumber1", "DrawNumber2", "DrawNumber3",
                "DrawNumber4", "DrawNumber5", "DrawNumber6", 
                "DrawNumber7", "DrawDate"
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
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
            r#"UPDATE lotpal."Cn649" SET 
                "DrawNumber1" = $1, "DrawNumber2" = $2,
                "DrawNumber3" = $3, "DrawNumber4" = $4,
                "DrawNumber5" = $5, "DrawNumber6" = $6,
                "DrawNumber7" = $7, "DrawDate" = $8
            WHERE "SerialNumber" = $9"#,
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
            r#"DELETE FROM lotpal."Cn649" WHERE "SerialNumber" = $1"#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

// Add this function to establish database connection
pub async fn establish_connection() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPool::connect(&database_url).await?;
    Ok(pool)
}