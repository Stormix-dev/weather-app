use axum::{extract::Query, Json};
use serde::Deserialize;

use crate::weather::{fetch_weather, WeatherResponse};

#[derive(Deserialize)]
pub struct WeatherQuery {
    city: String,
}

pub async fn get_weather(Query(params): Query<WeatherQuery>) -> Json<WeatherResponse> {
    let data = fetch_weather(&params.city).await.unwrap_or_else(|_| WeatherResponse {
        city: params.city.clone(),
        temperature: 0.0,
        description: "Errore".into(),
        humidity: 0,
    });

    Json(data)
}
