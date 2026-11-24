use crate::player;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::Rect};
use std::io;
use std::path::PathBuf;

#[derive(Default)]
enum Screen {
    #[default]
    List,
    Visualizer,
}

#[derive(Default)]
pub struct App {
    exit: bool,
    active_screen: Screen,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_events()?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                //KeyCode::Char(char) => self.exit(),
                KeyCode::Esc => self.exit(),
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let [top, main] = area.layout(&layout);
    }
}
