#![allow(unused)]
use crate::fetch_weather;
use crate::types::WeatherDetails;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::{border, line},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use tokio;

#[derive(Default, Debug)]
struct App {
    city: String,
    weather_details: Option<WeatherDetails>,
    exit: bool,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.exit();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.handle_weather_fetch();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.city.push(c);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                kind: KeyEventKind::Press,
                ..
            }) => {
                self.city.pop();
            }
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_weather_fetch(&mut self) {
        let city = self.city.clone();
        tokio::spawn(async move {
            let response = fetch_weather(&city).await;
            if response.status().is_success() {
                let weather_text = response.text().await.expect("Failed to read response text");
                let details: WeatherDetails = serde_json::from_str(&weather_text).unwrap();
            } else {
                println!(
                    "Failed to fetch weather data for {}: {}",
                    city,
                    response.status()
                );
            }
        });
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Weather TUI ").bold().underlined();
        let instruction =
            Line::from(" Type a city name and press Enter to get the weather. Press 'q' to quit. ")
                .italic();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instruction.centered())
            .border_set(border::ROUNDED);
        let content = Paragraph::new("City: ".to_string() + &self.city).block(block);
        let weather_info = if let Some(details) = &self.weather_details {
            format!(
                "City: {}\nTemperature: {:.2}°C\nMin Temp: {:.2}°C\nMax Temp: {:.2}°C\nHumidity: {}%\nPressure: {} hPa\nWind Speed: {:.2} m/s\nWind Direction: {}°\nCloudiness: {}%\nDescription: {}\n",
                details.name,
                details.main.temp - 273.15,
                details.main.temp_min - 273.15,
                details.main.temp_max - 273.15,
                details.main.humidity,
                details.main.pressure,
                details.wind.speed,
                details.wind.deg,
                details.clouds.all,
                details
                    .weather
                    .first()
                    .map_or("N/A", |w| w.description.as_str())
            )
        } else {
            "No weather data available.".to_string()
        };
        content.centered().render(area, buf);
        weather_info.render(area, buf);
    }
}

pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
