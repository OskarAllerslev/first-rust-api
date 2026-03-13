use axum::{routing::get, Router};
mod handlers;

// Vi bruger nu standard tokio i stedet for shuttle
#[tokio::main]
async fn main() {
    // Sætter vores router op med dine endpoints
    let app = Router::new()
        .route("/is_alive", get(handlers::alive_check))
        .route("/data/{ticker}", get(handlers::get_ticker_data))
        .route("/ema/{ticker}", get(handlers::get_ema));
    
    // Definerer adressen og porten
    let adresse = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(adresse).await.unwrap();
    
    println!("API kører nu på http://{}", adresse);
    
    // Starter selve serveren
    axum::serve(listener, app).await.unwrap();
}