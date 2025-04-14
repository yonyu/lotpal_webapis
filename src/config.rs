use dotenv::dotenv;
use std::env;

pub fn get_database_url() -> String {
    dotenv().ok(); // Load .env file
    env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env")
}