use crate::player;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Tabs, Widget};
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
    active_screen: usize,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Length(1)]);
        let [tabs, main] = frame.area().layout(&layout);

        self.render_tabs(frame, tabs);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                //KeyCode::Char(char) => self.exit(),
                KeyCode::Esc => self.exit(),
                KeyCode::Tab => self.active_screen = (self.active_screen + 1) % 2,
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Tabs::new(["Home", "List"]).select(self.active_screen), area);
    }
}
