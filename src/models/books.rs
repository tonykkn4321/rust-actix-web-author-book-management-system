use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: i32,           // Use i32 for ID
    pub title: String,
    pub year: i32,
    pub author_id: i32,    // Use i32 for author ID
}