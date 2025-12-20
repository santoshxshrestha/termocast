mod types;
mod ui;
use ui::tui;

// Function to fetch weather data from OpenWeatherMap API
async fn fetch_weather(city: &str) -> reqwest::Response {
    // Load API key from environment variable
    dotenv::dotenv().ok();

    // this should fail if the key is not found
    let key =
        dotenv::var("OPEN_WEATHER_API_KEY").expect("API key not found in environment variables");

    let base_url = "http://api.openweathermap.org/data/2.5/weather?";
    let complete_url = format!("{}q={}&appid={}", base_url, city, key);
    reqwest::get(&complete_url)
        .await
        .expect("Failed to send request")
}

#[tokio::main]
async fn main() {
    tui().unwrap();
}
