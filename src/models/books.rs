use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: i32, // Changed from Uuid to i32
    pub title: String,
    pub author_id: i32, // Changed from Uuid to i32
}