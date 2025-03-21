use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::{models::pieza::Part, services::parts_service::PartsService};

use super::create_parts::CreatePartRequest;

// Obtener todas las partes
pub async fn get_parts(State(service): State<Arc<PartsService>>) -> Json<Vec<Part>> {
    Json(service.get_all_parts())
}

// Obtener una parte por ID
pub async fn get_part_by_id(
    State(service): State<Arc<PartsService>>,
    Path(id): Path<i32>,
) -> Result<Json<Part>, (StatusCode, Json<serde_json::Value>)> {
    match service.get_part_by_id(id) {
        Some(part) => Ok(Json(part)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Part not found"})),
        )),
    }
}
pub async fn create_part(
    State(service): State<Arc<PartsService>>,
    Json(new_part): Json<CreatePartRequest>, // Recibe la parte desde el frontend
) -> Json<Part> {
    let part = service.create_part(new_part.name, new_part.stock); // Crear la parte
    Json(part)
}
