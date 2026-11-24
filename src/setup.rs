use std::path::PathBuf;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Rect;

use crate::track::Track;
use crate::{load_dir, walk};
//use std::io;
use color_eyre::Result;
use std::error::Error;

//type Result<T> = Result<T, Box<dyn Error>>;

pub struct Setup {
    first: bool,
    exit: bool,
    dir: Option<PathBuf>,
    bufs: Vec<PathBuf>,
    // tracks: Vec<Track>,
}

impl Setup {
    pub fn new() -> Self {
        //let dir: PathBuf = load_dir().unwrap();
        //let bufs: Vec<PathBuf> = walk(&dir);
        //let tracks: Vec<Track> = vec![];
        Self {
            first: true,
            exit: false,
            dir: None,
            bufs: Vec::new(),
            //tracks,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        let tracks: Vec<Track> = Vec::new();
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            self.handle_events()?;
        }
        Ok(tracks) //return the Vec once done, make sure to change this lol
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc => self.exit(),
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        //let content:
        //frame.render_widget(content, area);
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
