#![allow(unused)]
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use tokio;

fn render(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}

#[derive(Default, Debug)]
struct App {
    city: String,
    exit: bool,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
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
                todo!("fetch and display weather for {}", self.city);
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
        content.centered().render(area, buf);
    }
}

pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    // loop {
    //     terminal.draw(render).expect("failed to draw frame");
    //     // here matches will return true if the event is a key event
    //     if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
    //         break;
    //     }
    // }
    ratatui::restore();
    app_result
}
