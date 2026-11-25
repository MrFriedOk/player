use ratatui::Frame;
use ratatui::{DefaultTerminal, Terminal};

use crate::setup::Config;
use std::io::{Error, Result};

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
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {}
}
