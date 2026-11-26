use color_eyre::eyre::bail;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Modifier;
use ratatui::widgets::Gauge;
use ratatui::{DefaultTerminal, Terminal};

use crate::setup::Config;
use crate::track::Track;
use crate::walk;
use color_eyre::Result;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, SendError, TryRecvError, channel};
use std::thread;
use std::time::Duration;

pub struct Startup {
    exit: bool,
    config: Config,
    tracks: Vec<Track>,
    rx: Option<Receiver<Track>>,
    total: Option<f64>,
}

impl Startup {
    pub fn new(config: Config) -> Self {
        Self {
            exit: false,
            config,
            tracks: Vec::new(),
            rx: None,
            total: None,
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        //let mut tracks: Vec<Track> = Vec::new();
        self.load_tracks();
        while !self.exit {
            self.collect();
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(self.tracks)
    }

    fn render(&self, frame: &mut Frame) {
        let area: Rect = frame.area();

        let Some(total) = self.total else { return };
        let percent = ((self.tracks.len() as f64 / total) * 100.) as u16;

        let gauge = Gauge::default()
            .percent(percent)
            .label("loading tracks")
            .style(Modifier::DIM);

        frame.render_widget(gauge, area);
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn load_tracks(&mut self) {
        let (tx, rx) = channel::<Track>();

        let dir = self.config.dir.clone();

        thread::spawn(move || {
            let dir = dir;
            let Ok(dirs) = walk(&dir) else { return };
            for dir in dirs {
                if let Some(track) = Track::from_path(&dir) {
                    if tx.send(track).is_err() {
                        break;
                    }
                }
            }
        });

        self.rx = Some(rx);
    }

    fn collect(&mut self) {
        let Some(rx) = &self.rx else { return };

        loop {
            match rx.try_recv() {
                Ok(track) => {
                    //println!("{:?}", track);
                    self.tracks.push(track);
                }
                Err(TryRecvError::Empty) => continue, //println!("empty channel"), //continue,
                Err(TryRecvError::Disconnected) => break, //self.exit(),
            }
        }

        self.exit();
    }
}
