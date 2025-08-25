use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

