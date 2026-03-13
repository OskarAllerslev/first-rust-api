use axum::{routing::get, Router};
mod handlers;
mod types;
mod math;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/is_alive", get(handlers::alive_check))
        .route("/data/{ticker}", get(handlers::get_ticker_data))
        .route("/ema/{ticker}", get(handlers::get_ema));
    
    // Læser porten fra Render, eller bruger 3000 lokalt
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let adresse = format!("0.0.0.0:{}", port);
    
    let listener = tokio::net::TcpListener::bind(&adresse).await.unwrap();
    println!("API kører nu på {}", adresse);
    
    axum::serve(listener, app).await.unwrap();
}