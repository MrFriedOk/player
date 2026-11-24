use std::path::PathBuf;

use ratatui::DefaultTerminal;

use crate::track::Track;

pub struct Setup {
    first: bool,
    exit: bool,
    dir: PathBuf,
    bufs: Vec<PathBuf>,
    tracks: Vec<Track>,
}

impl Setup {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>, String> {
        while !self.exit {}
    }
}
