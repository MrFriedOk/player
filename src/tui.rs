use crate::player;
use crate::track::Track;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::{Block, Tabs, Widget};
use ratatui::{DefaultTerminal, Frame, symbols};
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::Rect};
use std::io;
use std::path::PathBuf;

pub struct App {
    exit: bool,
    active_screen: usize,
    tracks: Vec<Track>,
}

impl App {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            exit: false,
            active_screen: 0,
            tracks,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let v_layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
        let h_layout: Layout = Layout::horizontal([Constraint::Length(1), Constraint::Length(1)]);
        let [tabs, main] = frame.area().layout(&v_layout);
        let [left, right] = main.layout(&h_layout);

        self.render_tabs(frame, tabs);

        self.render_screen(frame, main);
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

    fn exit(&mut self) {
        self.exit = true;
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect) {
        let block: Block = Block::bordered();
        frame.render_widget(
            Tabs::new(["Home", "List"])
                .select(self.active_screen)
                .padding(" ", " ")
                .block(block),
            area,
        );
    }

    fn render_screen(&self, frame: &mut Frame, area: Rect) {
        let border = Block::bordered();

        frame.render_widget(border, area);
    }
}
