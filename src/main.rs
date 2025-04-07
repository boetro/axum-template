use axum::{
    routing::{get, get_service},
    Json, Router,
};
use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use std::env;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // Get host and port from environment variables or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    // build our application with routes
    let app = Router::new()
        .route("/api/hello", get(hello_world_api))
        .fallback(get_service(ServeDir::new("static")));

    // run our app with hyper, listening on the configured address
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Debug, Deserialize)]
struct ApiResponse {
    message: String,
}

static GREETINGS: [&str; 10] = [
    "Hello, world!",      // English
    "¡Hola, mundo!",      // Spanish
    "Bonjour, le monde!", // French
    "Hallo, Welt!",       // German
    "Ciao, mondo!",       // Italian
    "Olá, mundo!",        // Portuguese
    "Привет, мир!",       // Russian
    "你好，世界！",       // Chinese
    "こんにちは、世界！", // Japanese
    "안녕하세요, 세계!",  // Korean
];

async fn hello_world_api() -> Json<ApiResponse> {
    let greeting = GREETINGS
        .choose(&mut rand::rng())
        .unwrap_or(&"Hello, World!");
    Json(ApiResponse {
        message: greeting.to_string(),
    })
}
