use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub id: u64,
    pub title_id: u64,
    pub title: String,
    pub src: String,
    pub index: i32,
}
