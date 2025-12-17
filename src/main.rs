#![allow(unused)]
use dotenv;
use reqwest;
use serde;
use serde_json;
use tokio;
mod types;
use types::WeatherDetails;

async fn fetch_weather(city: &str, key: String) -> reqwest::Response {
    let base_url = "http://api.openweathermap.org/data/2.5/weather?";
    let complete_url = format!("{}q={}&appid={}", base_url, city, key);
    let response = reqwest::get(&complete_url)
        .await
        .expect("Failed to send request");
    return response;
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let key = dotenv::var("OPEN_WEATHER_API_KEY").unwrap();
    let city = "Butwal";
    let weather_response = fetch_weather(city, key).await;
    let weather_text = weather_response
        .text()
        .await
        .expect("Failed to read response text");

    let details: WeatherDetails = serde_json::from_str(&weather_text).unwrap();
    println!("Weather Data: {:#?}", details);
}
