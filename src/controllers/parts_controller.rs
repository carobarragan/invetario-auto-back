use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::services::parts_service::{PartsService, PartsServiceTrait};
use crate::{
    models::parts::{Part, PartsError},
    services::parts_service::DynPartsService,
};

// Implementación de IntoResponse para manejar errores
impl IntoResponse for PartsError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            PartsError::NotFound => (StatusCode::NOT_FOUND, "Parte no encontrada".to_string()),
            PartsError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            PartsError::Io(_) | PartsError::Json(_) => {
                let message = self.to_string();
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };

        (
            status,
            Json(json!({ "success": false, "error": error_message })),
        )
            .into_response()
    }
}

// Trait para el controlador
pub trait PartsControllerTrait {
    fn config_endpoints() -> Router;
}

// Implementación del controlador
pub struct PartsController {}

impl PartsControllerTrait for PartsController {
    fn config_endpoints() -> Router {
        let routes = Arc::new(PartsService::default()) as DynPartsService;

        Router::new().nest("/api/parts", create_routes(routes.clone()))
    }
}

// Se pasa el estado directamente y se aplica a las rutas
fn create_routes(state: DynPartsService) -> Router {
    Router::new()
        .route("/", get(get_all_parts))
        .route("/", post(add_new_part))
        .route("/:id", get(get_part))
        .route("/:id", put(update_existing_part))
        .route("/:id", delete(delete_existing_part))
        .with_state(state)
}

// Handlers

async fn get_all_parts(
    State(service): State<DynPartsService>,
) -> Result<Json<Vec<Part>>, PartsError> {
    let parts = service.get_all().await?;
    Ok(Json(parts))
}

async fn get_part(
    State(service): State<DynPartsService>,
    Path(id): Path<u32>,
) -> Result<Json<Part>, PartsError> {
    let part = service.get_by_id(id).await?;
    Ok(Json(part))
}

async fn add_new_part(
    State(service): State<DynPartsService>,
    Json(new_part): Json<Part>,
) -> Result<(StatusCode, Json<serde_json::Value>), PartsError> {
    let added_part = service.add(new_part).await?;
    Ok((
        StatusCode::CREATED,
        Json(json!({ "success": true, "part": added_part })),
    ))
}

async fn update_existing_part(
    State(service): State<DynPartsService>,
    Path(id): Path<u32>,
    Json(updated_part): Json<Part>,
) -> Result<Json<serde_json::Value>, PartsError> {
    let updated = service.update(id, updated_part).await?;
    Ok(Json(json!({ "success": true, "part": updated })))
}

async fn delete_existing_part(
    State(service): State<DynPartsService>,
    Path(id): Path<u32>,
) -> Result<Json<serde_json::Value>, PartsError> {
    service.delete(id).await?;
    Ok(Json(json!({ "success": true })))
}
