use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{DefaultTerminal, Terminal};

use crate::setup::Config;
use crate::track::Track;
use crate::walk;
use color_eyre::Result;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub struct Startup {
    exit: bool,
    config: Config,
    tracks: Vec<Track>,
}

impl Startup {
    pub fn new(config: Config) -> Self {
        Self {
            exit: false,
            config,
            tracks: Vec::new(),
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        //let mut tracks: Vec<Track> = Vec::new();
        self.load_tracks()?;
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;
        }
        Ok(self.tracks)
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
        let (tx, rx) = mpsc::channel::<Track>();
        let dir: PathBuf = self.config.dir.clone();

        thread::spawn(move || {
            let dirs: Vec<PathBuf> = walk(&dir).expect("couldn't walk directory");

            for dir in dirs {
                //self.tracks.push(Track::from_path(dir).unwrap());
                //tx.send(Track::from_path(dir));
                let Some(track) = Track::from_path(&dir) else {
                    continue;
                };
                if tx.send(track).is_err() {
                    break;
                }
            }

            //return Ok(Vec::new());
        });

        while let Ok(track) = rx.try_recv() {
            println!("{:?}", track);
            self.tracks.push(track);
        }
        self.exit();

        Ok(())
    }
}
