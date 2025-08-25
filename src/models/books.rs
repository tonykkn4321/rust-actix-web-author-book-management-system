use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author_id: Uuid,
}