use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Terminal};

use crate::setup::Config;
use crate::track::Track;
use crate::walk;
use std::io::{Error, Result};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub struct Startup {
    exit: bool,
    config: Config,
}

impl Startup {
    pub fn new(config: Config) -> Self {
        Self {
            exit: false,
            config,
        }
    }
    pub fn run(self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        let mut tracks: Vec<Track> = Vec::new();
        self.load_tracks();
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(tracks)
    }

    fn render(&self, frame: &mut Frame) {
        let area: Rect = frame.area();

        let content_area: Rect = Rect {
            x: area.width / 4,
            y: area.width / 4,
            width: area.width / 2,
            height: area.height / 2,
        };
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn load_tracks(&self) -> Result<Vec<Track>> {
        let (tx, rx) = mpsc::channel();
        let dir: PathBuf = self.config.dir.clone();

        thread::spawn(|| {
            let dirs: Vec<PathBuf> = walk(&dir)?;

            return Ok(Vec::new());
        });
    }
}
