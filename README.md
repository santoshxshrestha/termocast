# termocast

A learning project for building Terminal User Interfaces (TUI) in Rust, demonstrating event handling, async operations, and real-time data fetching. This project fetches weather data from OpenWeatherMap API and displays it in a terminal-based interface.

## Project Purpose

This is a learning project focused on:

- Building TUIs with Ratatui
- Implementing event listeners for keyboard input
- Working with async/await patterns in Rust
- Integrating async operations with synchronous UI rendering
- Managing shared state across async boundaries using Arc and Mutex

## Features

- Interactive terminal-based weather application
- Real-time keyboard event handling
- Asynchronous API data fetching
- Clean separation of concerns (types, UI, main logic)

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

### Key Learning Concepts

#### 1. TUI with Ratatui (src/ui.rs)

The UI is built using the Ratatui framework:
- `App` struct holds application state
- `run()` method implements the main event loop
- `draw()` renders the UI on each frame
- `Widget` trait implementation for custom rendering

```rust
// Main event loop in ui.rs:25-34
while !self.exit {
    terminal.draw(|frame| self.draw(frame))?;
    self.handle_events()?;
}
````

#### 2. Event Listeners (src/ui.rs:40-80)

Keyboard events are handled using Crossterm's event polling:

- `poll()` checks for events without blocking (ui.rs:41)
- Pattern matching on `KeyEvent` handles different keys
- 'q' to quit (ui.rs:44-49)
- 'Enter' to fetch weather (ui.rs:50-61)
- Character input for city name (ui.rs:62-68)
- 'Backspace' to delete characters (ui.rs:69-75)

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

#### 3. Async Integration (src/ui.rs:86-106)

The challenge: UI rendering is synchronous, but API fetching is async.

**Solution**: Spawn async tasks with shared state:

- Weather data stored in `Arc<Mutex<Option<WeatherDetails>>>` (ui.rs:19)
- `tokio::spawn()` runs fetch in background (ui.rs:89)
- Mutex allows safe concurrent access
- UI polls the shared state on each render (ui.rs:120-124)

```rust
fn handle_weather_fetch(&mut self) {
    let city = self.city.clone();
    let weather_details_arc = Arc::clone(&self.weather_details);

    // Spawn async task that doesn't block UI
    tokio::spawn(async move {
        let response = fetch_weather(&city).await;
        // Update shared state when done
        let mut weather_details = weather_details_arc
            .lock()
            .expect("weather_details poisoned");
        *weather_details = Some(details);
    });
}
```

#### 4. Async Runtime (src/main.rs:21-24)

- `#[tokio::main]` macro creates async runtime
- Allows calling async functions from sync code
- Enables background task execution

#### 5. Data Structures (src/types.rs)

Serde-powered deserialization of API responses:

- `WeatherDetails` - Main weather data
- `MainReadings` - Temperature, humidity, pressure
- `WindInfo` - Wind speed and direction
- `CloudCover` - Cloudiness percentage

## Usage

1. Launch the application
2. Type a city name (e.g., "London")
3. Press Enter to fetch weather data
4. View the results in the terminal
5. Press 'q' to quit

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
