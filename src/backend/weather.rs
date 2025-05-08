use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
pub struct WeatherResponse {
    pub city: String,
    pub temperature: f64,
    pub description: String,
    pub humidity: u8,
}

#[derive(Deserialize)]
struct WeatherAPIResponse {
    current: Current,
}

#[derive(Deserialize)]
struct Current {
    temp_c: f64,
    humidity: u8,
    condition: Condition,
}

#[derive(Deserialize)]
struct Condition {
    text: String,
}

pub async fn fetch_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
    let api_key = env::var("WEATHER_API_KEY").expect("API key mancante");
    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&lang=it",
        api_key, city
    );

    let res = reqwest::get(&url).await?.json::<WeatherAPIResponse>().await?;

    Ok(WeatherResponse {
        city: city.to_string(),
        temperature: res.current.temp_c,
        description: res.current.condition.text.clone(),
        humidity: res.current.humidity,
    })
}


// MOCK di esempio
/*
    pub async fn fetch_weather(city: &str) -> Result<WeatherResponse, reqwest::Error> {
        Ok(WeatherResponse {
            city: city.to_string(),
            temperature: 20.5,
            description: "Soleggiato (mock)".into(),
            humidity: 42,
        })
    }
*/

