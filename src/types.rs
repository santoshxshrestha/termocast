use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct WeatherDetails {
    name: String,
    dt: u64,
    weather: Vec<WeatherCondition>,
    main: MainReadings,
    wind: WindInfo,
    clouds: CloudCover,
}

#[derive(Debug, Deserialize)]
pub struct WeatherCondition {
    description: String,
}

#[derive(Debug, Deserialize)]
pub struct MainReadings {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
    humidity: u8,
    pressure: u16,
}

#[derive(Debug, Deserialize)]
pub struct WindInfo {
    speed: f32,
    deg: u16,
}

#[derive(Debug, Deserialize)]
pub struct CloudCover {
    all: u8,
}
