# termocast

A learning project for building Terminal User Interfaces (TUI) in Rust, demonstrating event handling, async operations, and real-time data fetching. This project fetches weather data from OpenWeatherMap API and displays it in a terminal-based interface with beautiful ASCII art weather visualizations.

## Project Purpose

This is a learning project focused on:

- Building TUIs with Ratatui
- Implementing event listeners for keyboard input
- Working with async/await patterns in Rust
- Integrating async operations with synchronous UI rendering
- Managing shared state across async boundaries using Arc and Mutex
- Creating dynamic ASCII art based on real-time data

## Features

- Interactive terminal-based weather application
- Real-time keyboard event handling
- Asynchronous API data fetching
- Dynamic ASCII art weather visualizations (day/night variants)
- Clean separation of concerns (types, UI, art, main logic)
- Loading states and error handling

## Setup

### Prerequisites

- Rust toolchain
- OpenWeatherMap API key (get one at https://openweathermap.org/api)

### Configuration

Put your API key in a `.env` file:

```
OPEN_WEATHER_API_KEY=your_key
```

### Running the Project

From source:

```bash
cargo run
```

From binary:

```bash
export OPEN_WEATHER_API_KEY=your_key && termocast
```

or, what ever shell you use you can set env variable accordingly

````

## Project Architecture

### File Structure

- `src/main.rs` - Entry point and async weather fetching logic
- `src/ui.rs` - TUI implementation with Ratatui
- `src/types.rs` - Data structures for weather API responses
- `src/art.rs` - ASCII art system with day/night weather visualizations

### Key Learning Concepts

#### 1. TUI with Ratatui (src/ui.rs)

The UI is built using the Ratatui framework:
- `App` struct holds application state including weather data, ASCII art, and loading flags
- `run()` method implements the main event loop (ui.rs:29-38)
- `draw()` renders the UI on each frame (ui.rs:40-42)
- `Widget` trait implementation for custom rendering (ui.rs:117-180)

```rust
// Main event loop in ui.rs:33-36
while !self.exit {
    terminal.draw(|frame| self.draw(frame))?;
    self.handle_events()?;
}
````

#### 2. Event Listeners (src/ui.rs:44-85)

Keyboard events are handled using Crossterm's event polling:

- `poll()` checks for events without blocking (ui.rs:45)
- Pattern matching on `KeyEvent` handles different keys
- 'q' to quit (ui.rs:47-53)
- 'Enter' to fetch weather (ui.rs:54-66)
- Character input for city name (ui.rs:67-73)
- 'Backspace' to delete characters (ui.rs:74-80)

**Note:** The quit key is lowercase 'q' only. If you want to search for a city containing the letter 'q' (like "Qatar" or "Iraq"), you can use capital 'Q' instead.

```rust
// Non-blocking event polling
if poll(Duration::from_micros(1))? {
    match event::read()? {
        Event::Key(key_event) => {
            // Handle keyboard input
        }
        _ => {}
    }
}
```

#### 3. Async Integration (src/ui.rs:91-114)

The challenge: UI rendering is synchronous, but API fetching is async.

**Solution**: Spawn async tasks with shared state:

- Weather data stored in `Arc<Mutex<Option<WeatherDetails>>>` (ui.rs:21)
- Fetching status tracked with `Arc<AtomicBool>` (ui.rs:24)
- `tokio::spawn()` runs fetch in background (ui.rs:95)
- Mutex allows safe concurrent access
- UI polls the shared state on each render (ui.rs:128-163)

```rust
fn handle_weather_fetch(&mut self) {
    let city = self.city.clone();
    let weather_details_arc = Arc::clone(&self.weather_details);
    let isfetching_arc = Arc::clone(&self.isfetching);

    // Spawn async task that doesn't block UI
    tokio::spawn(async move {
        let response = fetch_weather(&city).await;
        if response.status().is_success() {
            // Parse and update shared state
            let details: WeatherDetails = 
                serde_json::from_str(&weather_text).expect("Failed to parse JSON");
            *weather_details = Some(details);
        }
        isfetching_arc.store(false, Ordering::SeqCst);
    });
}
```

#### 4. Async Runtime (src/main.rs:22-27)

- `#[tokio::main]` macro creates async runtime
- Allows calling async functions from sync code
- Enables background task execution

#### 5. Data Structures (src/types.rs)

Serde-powered deserialization of API responses:

- `WeatherDetails` - Main weather data with timestamp and timezone
- `SysInfo` - Sunrise and sunset times for day/night detection
- `WeatherCondition` - Weather description for ASCII art selection
- `MainReadings` - Temperature (min/max), humidity, pressure
- `WindInfo` - Wind speed and direction
- `CloudCover` - Cloudiness percentage

#### 6. ASCII Art System (src/art.rs)

Dynamic weather visualization system:

- `AsciiArt` struct with HashMap of weather conditions
- `ArtPair` struct containing day and night art variants
- Supports multiple conditions: sunny, cloudy, rainy, stormy, snowy, smoke/haze/fog
- `get_art()` method selects appropriate art based on weather description and time of day
- Day/night detection using sunrise/sunset times from API (ui.rs:189-198)

## Usage

1. Launch the application
2. Type a city name (e.g., "London")
3. Press Enter to fetch weather data
4. View the results with ASCII art visualization in the terminal
5. The art adapts to day/night based on local sunrise/sunset times
6. Press 'q' to quit

## Application States

The app handles multiple states gracefully:
- **Initial State**: Prompts user to enter a city name
- **Loading State**: Shows "Fetching weather data..." while API call is in progress
- **Success State**: Displays weather data with appropriate ASCII art
- **Error State**: Shows "City not found or error fetching data" for invalid cities or API failures

## Technologies Used

- **Ratatui** - Terminal UI framework
- **Crossterm** - Cross-platform terminal manipulation
- **Tokio** - Async runtime
- **Reqwest** - HTTP client for API calls
- **Serde** - JSON deserialization
- **Dotenv** - Environment variable management

## Learning Outcomes

This project teaches:

1. How to build responsive TUIs that don't freeze during I/O
2. Event-driven architecture in terminal applications
3. Bridging sync UI code with async background tasks
4. Thread-safe state sharing with Arc and Mutex
5. Non-blocking event polling patterns
6. Using AtomicBool for lock-free state flags
7. Dynamic content selection based on real-time data
8. Time-zone aware calculations for day/night detection
9. Error handling in async contexts
10. Structuring larger Rust projects with multiple modules
