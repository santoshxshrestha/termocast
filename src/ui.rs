use crate::art::AsciiArt;
use crate::fetch_weather;
use crate::types::WeatherDetails;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, poll};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{io, time::Duration};

#[derive(Default, Debug)]
struct App {
    city: String,
    weather_details: Arc<Mutex<Option<WeatherDetails>>>,
    art: AsciiArt,
    exit: bool,
    isfetching: Arc<AtomicBool>,
    fetched_once: bool,
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
        if poll(Duration::from_micros(1))? {
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
                    if self.city.is_empty() {
                        return Ok(());
                    }
                    self.handle_weather_fetch();
                    self.city.clear();
                    self.fetched_once = true;
                    self.isfetching.store(true, Ordering::SeqCst);
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
            }
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_weather_fetch(&mut self) {
        let city = self.city.clone();
        let weather_details_arc = Arc::clone(&self.weather_details);
        let isfetching_arc = Arc::clone(&self.isfetching);
        tokio::spawn(async move {
            let response = fetch_weather(&city).await;
            if response.status().is_success() {
                let weather_text = response.text().await.expect("Failed to read response text");
                let details: WeatherDetails =
                    serde_json::from_str(&weather_text).expect("Failed to parse JSON");
                let mut weather_details = weather_details_arc
                    .lock()
                    .expect("weather_details poisoned");
                isfetching_arc.store(false, Ordering::SeqCst);
                *weather_details = Some(details);
            } else {
                let mut weather_details = weather_details_arc
                    .lock()
                    .expect("weather_details poisoned");
                isfetching_arc.store(false, Ordering::SeqCst);
                *weather_details = None;
            }
        });
    }
}

impl Widget for &App {
    // The render method runs every time the UI needs to be redrawn so no any mutations should be done here
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" TermoCast ").bold().underlined();
        let instruction =
            Line::from(" Type a city name and press Enter. Press 'q' to quit. ").italic();
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instruction.centered())
            .border_set(border::ROUNDED);

        let weather_info = if let Some(details) = &self
            .weather_details
            .lock()
            .expect("weather_details poisoned")
            .as_ref()
        {
            format!(
                "{}\n\nCity: {}\nTemperature: {:.2}°C\nMin Temp: {:.2}°C\nMax Temp: {:.2}°C\nHumidity: {}%\nPressure: {} hPa\nWind Speed: {:.2} m/s\nWind Direction: {}°\nCloudiness: {}%\nDescription: {}\n",
                self.art.get_art(
                    details
                        .weather
                        .first()
                        .map_or("N/A", |w| w.description.as_str()),
                    is_day(details)
                ),
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
        } else if self.isfetching.load(Ordering::SeqCst) {
            "\nFetching weather data...".to_string()
        } else if self.fetched_once {
            "\nCity not found or error fetching data.".to_string()
        } else {
            "\nPlease enter a city name to get the weather information.".to_string()
        };

        // Render city input box
        Paragraph::new(self.city.as_str())
            .block(
                Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("City"),
            )
            .centered()
            .render(area, buf);

        Paragraph::new(weather_info.as_str())
            .block(block)
            .centered()
            .render(area, buf);
    }
}

pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

pub fn is_day(details: &WeatherDetails) -> bool {
    // adding the offset to convert to local time though it's not strictly necessary for this comparison
    let current_time = details.dt + details.timezone;

    let sunrise_time = details.sys.sunrise + details.timezone;

    let sunset_time = details.sys.sunset + details.timezone;

    current_time >= sunrise_time && current_time < sunset_time
}
