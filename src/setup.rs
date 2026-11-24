use std::path::PathBuf;

use color_eyre::config::Frame;
use ratatui::DefaultTerminal;

use crate::{load_dir, walk}();
use crate::track::Track;
//use std::io;
use std::error::Error;

//type Result<T> = Result<T, Box<dyn Error>>;

pub struct Setup {
    first: bool,
    exit: bool,
    dir: PathBuf,
    bufs: Vec<PathBuf>,
    tracks: Vec<Track>,
}

impl Setup {
    pub fn new() -> Self {
        Self { 
            first: true,
            exit: false,
            dir: load_dir().unwrap(),
            bufs: walk(load_dir().unwrap()),
            tracks: vec![for buf in ]

        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>, String> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame));

            self.handle_events()?;
        }
        Ok(Vec::new()) //return the Vec once done, make sure to change this lol
    }

    fn handle_events(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {}
}
