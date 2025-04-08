use axum::http::HeaderValue;
use axum::{Router, ServiceExt};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{AllowOrigin, CorsLayer};

mod controllers;
mod models;
mod repositories;
mod services;

use controllers::parts_controller::PartsControllerTrait;

#[tokio::main]
async fn main() {
    // Iniciar el servidor
    start_api_server().await;
}

/// Inicia el servidor API
async fn start_api_server() {
    println!("Iniciando servidor Axum...");

    // Configurar CORS con orígenes permitidos
    let cors = CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .allow_origin(AllowOrigin::list([
            "https://stocks-parts-ui.vercel.app"
                .parse::<HeaderValue>()
                .unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        ]));

    // Configurar las rutas directamente desde PartsController
    let app = controllers::parts_controller::PartsController::config_endpoints().layer(cors); // Aplicar CORS como middleware

    // Configurar el servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Servidor corriendo en http://{}", addr);

    // Iniciar el servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
