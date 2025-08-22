use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Author {
    pub id: Uuid,
    pub name: String,
    pub bio: String,
}
