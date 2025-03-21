use crate::controllers::create_parts::create_part;
use crate::controllers::parts_controller::{get_part_by_id, get_parts};
use crate::services::parts_service::PartsService;
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method};
use axum::{
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod controllers; // Tus controladores
mod models;
mod repositories; // Tu repositorio
mod services; // Tu servicio

#[tokio::main]
async fn main() {
    // Crear el servicio y envolverlo en Arc para compartirlo entre hilos
    let parts_service = Arc::new(PartsService::new());
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE]);

    // Configuración de las rutas usando los controladores
    let app = Router::new()
        .route("/api/parts", get(get_parts)) // Obtener todas las partes
        .route("/api/parts/:id", get(get_part_by_id)) // Obtener una parte por ID
        .route("/api/parts", post(create_part)) // Crear una parte
        .layer(cors) // Añadir el middleware CORS
        .with_state(parts_service); // Inyectar el servicio compartido

    // Configuración del servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Servidor corriendo en http://127.0.0.1:3000");

    // Iniciar el servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
