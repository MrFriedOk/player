use color_eyre::eyre::bail;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Terminal};

use crate::setup::Config;
use crate::track::Track;
use crate::walk;
use color_eyre::Result;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, TryRecvError, channel};
use std::thread;
use std::time::Duration;

pub struct Startup {
    exit: bool,
    config: Config,
    tracks: Vec<Track>,
    rx: Option<Receiver<Track>>,
    finished: bool,
}

impl Startup {
    pub fn new(config: Config) -> Self {
        Self {
            exit: false,
            config,
            tracks: Vec::new(),
            rx: None,
            finished: false,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        //let mut tracks: Vec<Track> = Vec::new();
        self.load_tracks()?;
        while !self.exit {
            self.collect();
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(self.tracks)
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
    fn load_tracks(&mut self) -> Result<()> {
        let (tx, rx) = channel::<Track>();

        let dirs = walk(&self.config.dir)?;

        thread::spawn(move || -> Result<()> {
            for dir in &dirs {
                //tx.send(Track::from_path(dir))
                if let Some(t) = Track::from_path(dir) {
                    tx.send(t)?
                }
            }
            //self.finished = true;
            Ok(())
        });

        self.rx = Some(rx);
        //let dirs = walk(&self.config.dir)?;
        //for dir in dirs {
        //    if let Some(track) = Track::from_path(&dir) {
        //        self.tracks.push(track);
        //    } else {
        //        continue;
        //    }
        //}
        //self.exit();
        Ok(())
    }

    fn collect(&mut self) -> Result<()> {
        let Some(rx) = &self.rx else {
            bail!("no receiver")
        };

        loop {
            match rx.try_recv() {
                Ok(track) => self.tracks.push(track),
                Err(TryRecvError::Empty) => continue,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        Ok(())
    }
}
