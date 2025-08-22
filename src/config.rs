use std::env;
use dotenvy::dotenv;

pub fn get_database_url() -> String {
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

    if app_env != "production" {
        dotenv().ok();
    }

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
