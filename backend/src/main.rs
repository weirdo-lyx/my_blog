use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

// Import the modules we created.
mod database;
mod handlers;
mod models;

// Use the functions and structs from our modules.
use handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file.
    dotenvy::dotenv().ok();

    // Get the database URL from the environment.
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Initialize the database pool.
    let pool = database::init_pool(&db_url).await?;
    let app_state = AppState { pool };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Define the application routes, using handlers from the `handlers` module.
    let app = Router::new()
        .route("/posts", get(get_posts).post(create_post))
        .route("/posts/{id}", get(get_post).put(update_post).delete(delete_post))
        .with_state(app_state)
        .layer(cors);

    // Start the server.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}