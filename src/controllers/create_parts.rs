use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::{models::pieza::Part, services::parts_service::PartsService};

// Estructura para recibir los datos del POST
#[derive(Deserialize, Serialize)]
pub struct CreatePartRequest {
    pub name: String,
    pub stock: u32,
}

// Crear una nueva parte
pub async fn create_part(
    State(service): State<Arc<PartsService>>,
    Json(payload): Json<CreatePartRequest>, // Recibimos la nueva parte como JSON
) -> Result<Json<Part>, (StatusCode, Json<serde_json::Value>)> {
    // Llamamos al servicio para crear la parte
    let new_part = service.create_part(payload.name, payload.stock);

    // Respondemos con la parte creada
    Ok(Json(new_part))
}
