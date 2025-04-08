use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Part {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub price: f64,
    pub brand: String,
    pub condition: String,
}

#[derive(Debug, Error)]
pub enum PartsError {
    #[error("Part not found")]
    NotFound,

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}