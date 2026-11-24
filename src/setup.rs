use std::path::PathBuf;

use ratatui::DefaultTerminal;

use crate::track::Track;
//use std::io;
use std::error::Error;

type Result<T, E> = Result<T, Box<dyn Error>>;

pub struct Setup {
    first: bool,
    exit: bool,
    dir: PathBuf,
    bufs: Vec<PathBuf>,
    tracks: Vec<Track>,
}

impl Setup {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame));

            self.handle_events()?;
        }
        Ok(Vec::new())
    }

    fn handle_events(&mut self) -> Result<()> {
        Ok(())
    }
}
