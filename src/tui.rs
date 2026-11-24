use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame};
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::Rect};
use std::io;

#[derive(Default)]
pub struct App {
    exit: bool,
}

pub fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut query = String::new();
    loop {
        terminal.draw(|frame| draw(frame, &query))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Char(c) => query.push(c),
                KeyCode::Backspace => {
                    query.pop();
                }
                _ => {}
            }
        }
    }
}

pub fn draw(frame: &mut Frame, query: &str) {
    let tabs = Tabs::new();
    let bar = Paragraph::new(query).block(Block::bordered().title("Search"));
    frame.render_widget(bar, frame.area());
}
