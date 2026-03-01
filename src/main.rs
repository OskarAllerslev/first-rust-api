

use axum:: {
    routing::get, 
    Router,
};
mod handlers;



// and this is the localhost interface
#[tokio::main]
async fn main() {
    // this should  show our alive check funtion in the api
    let app = Router::new()
        .route("/is_alive", get(handlers::alive_check))
        .route("/data/{ticker}", get(handlers::get_ticker_data));
    
    let adresse = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(adresse).await.unwrap();
    

    axum::serve(listener,app).await.unwrap();
}