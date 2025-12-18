use dotenv;
use reqwest;
use serde_json;
use tokio;

mod types;
mod ui;
use types::WeatherDetails;
use ui::tui;

async fn fetch_weather(city: &str, key: String) -> reqwest::Response {
    let base_url = "http://api.openweathermap.org/data/2.5/weather?";
    let complete_url = format!("{}q={}&appid={}", base_url, city, key);
    reqwest::get(&complete_url)
        .await
        .expect("Failed to send request")
}

#[tokio::main]
async fn main() {
    // Load API key from environment variable
    dotenv::dotenv().ok();

    // this should fail if the key is not found
    let key =
        dotenv::var("OPEN_WEATHER_API_KEY").expect("API key not found in environment variables");

    let city = "Butwal";
    let weather_response = fetch_weather(city, key).await;

    let weather_text = weather_response
        .text()
        .await
        .expect("Failed to read response text");

    let _details: WeatherDetails = serde_json::from_str(&weather_text).unwrap();
    // println!("{:#?}", details);
    //
    //
    tui().unwrap();
}
