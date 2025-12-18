#![allow(unused)]
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct WeatherDetails {
    pub name: String,
    pub dt: u64,
    pub weather: Vec<WeatherCondition>,
    pub main: MainReadings,
    pub wind: WindInfo,
    pub clouds: CloudCover,
}

#[derive(Debug, Deserialize)]
pub struct WeatherCondition {
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct MainReadings {
    pub temp: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub humidity: u8,
    pub pressure: u16,
}

#[derive(Debug, Deserialize)]
pub struct WindInfo {
    pub speed: f32,
    pub deg: u16,
}

#[derive(Debug, Deserialize)]
pub struct CloudCover {
    pub all: u8,
}
