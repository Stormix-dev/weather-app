mod handlers;
mod weather;

use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::cors::{CorsLayer, Any};  
use dotenvy::dotenv; 
use std::env; 

#[tokio::main]
async fn main() {
    dotenv().ok(); // Carica le variabili d'ambiente

    // Verifica se la variabile d'ambiente è presente (opzionale: per il debug)
    let api_key = env::var("WEATHER_API_KEY").expect("API key mancante");

    println!("Chiave API caricata: {}", api_key);  // Puoi rimuovere questa riga dopo aver verificato che funzioni


    let cors = CorsLayer::new().allow_origin(Any);  

    let app = Router::new()
        .route("/weather", get(handlers::get_weather))
        .layer(cors);  

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server running on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

