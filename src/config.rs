use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_app_env() -> String {
    env::var("APP_ENV").unwrap_or_else(|_| "development".to_string())
}