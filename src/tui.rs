use crate::player;
use crate::track::Track;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier};
use ratatui::widgets::{Block, List, ListItem, ListState, Tabs, Widget};
use ratatui::{DefaultTerminal, Frame, symbols};
use ratatui::{backend::CrosstermBackend, buffer::Buffer, layout::Rect};
use std::io;
use std::path::PathBuf;

pub struct App {
    exit: bool,
    active_screen: usize,
    tracks: Vec<Track>,
    select: usize,
}

impl App {
    pub fn new(tracks: Vec<Track>) -> Self {
        Self {
            exit: false,
            active_screen: 0,
            tracks,
            select: 0,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        //println!("{:?}", &self.tracks);
        while !self.exit {
            let mut state = ListState::default().with_selected(Some(self.select));
            terminal.draw(|frame| self.render(frame, &mut state))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame, state: &mut ListState) {
        let v_layout = Layout::vertical([Constraint::Percentage(7), Constraint::Min(0)]);
        let h_layout: Layout =
            Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(77)]);
        let [tabs, main] = frame.area().layout(&v_layout);
        let [left, right] = main.layout(&h_layout);

        self.render_tabs(frame, tabs);

        self.render_songs(frame, left, state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                //KeyCode::Char(char) => self.exit(),
                KeyCode::Esc => self.exit(),
                KeyCode::Tab => self.active_screen = (self.active_screen + 1) % 2,
                KeyCode::Down => {
                    if self.select == self.tracks.len() + 1 {
                        self.select = 0
                    } else {
                        self.select += 1
                    }
                } //self.select += 1,
                KeyCode::Up => {
                    if self.select == 0 {
                        self.select = self.tracks.len() - 1
                    } else {
                        self.select -= 1
                    }
                } //self.select -= 1,
                KeyCode::Enter => println!("{:?}", self.tracks[self.select]),
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
                .block(block),
            area,
        );
    }

    fn render_songs(&self, frame: &mut Frame, area: Rect, state: &mut ListState) {
        let border = Block::bordered();
        let items: Vec<ListItem> = self
            .tracks
            .iter()
            .map(|track| ListItem::new(track.label.clone()))
            .collect();
        //let sections = Layout::vertical([Constraint::Percentage(20), Constraint::Min(0)]);
        let list = List::new(items)
            .highlight_style(Modifier::REVERSED)
            .style(Color::Rgb(255, 255, 255));

        frame.render_stateful_widget(list, area, state);
    }
}
