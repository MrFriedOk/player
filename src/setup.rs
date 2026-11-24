use std::path::PathBuf;

use ratatui::DefaultTerminal;
use ratatui::Frame;

use crate::track::Track;
use crate::{load_dir, walk};
//use std::io;
use color_eyre::Result;
use std::error::Error;

//type Result<T> = Result<T, Box<dyn Error>>;

pub struct Setup {
    first: bool,
    exit: bool,
    dir: PathBuf,
    bufs: Vec<PathBuf>,
    // tracks: Vec<Track>,
}

impl Setup {
    pub fn new() -> Self {
        let dir: PathBuf = load_dir().unwrap();
        let bufs: Vec<PathBuf> = walk(&dir);
        //let tracks: Vec<Track> = vec![];
        Self {
            first: true,
            exit: false,
            dir,
            bufs,
            //tracks,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            self.handle_events()?;
        }
        Ok(Vec::new()) //return the Vec once done, make sure to change this lol
    }

    fn handle_events(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {}
}
