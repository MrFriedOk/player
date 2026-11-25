use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Terminal};

use crate::setup::Config;
use crate::track::Track;
use crate::walk;
use std::io::{Error, Result};
use std::path::PathBuf;
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
        load_tracks(self.config.dir);
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
}
fn load_tracks(dir: PathBuf) -> Vec<Track> {
    thread::spawn(|| {
        let dir = dir;
        for path in walk(&dir) {}
    });
}
