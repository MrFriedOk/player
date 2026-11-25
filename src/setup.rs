use std::env::home_dir;
//use std::fmt::Error;
use std::fs::File;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use directories::BaseDirs;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ron::from_str;
use ron::ser::PrettyConfig;
use ron::ser::to_string_pretty;
use serde::Deserialize;
use serde::Serialize;

use crate::probe_dir;
use crate::track::Track;
//use std::io;
use std::io::Result;
//use color_eyre::Result;

//type Result<T> = Result<T, Box<dyn Error>>;

pub enum Status {
    CheckingDir { text: String },
    Failed { text: String },
    Success { text: String },
    Confirm { text: String, dir: PathBuf },
}

pub struct Setup {
    first: bool,
    exit: bool,
    dir: Option<PathBuf>,
    bufs: Vec<PathBuf>,
    // tracks: Vec<Track>,
    input: String,
    status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dir: PathBuf,
}

impl Config {
    fn from_path(path: PathBuf) -> Result<Self> {
        let dir = read_to_string(path)?;
        let dir = ron::from_str(&dir).unwrap();
        Ok(Self { dir })
    }
}

fn load_config() -> Result<Option<Config>> {
    if let Some(base) = BaseDirs::new() {
        let config_dir = base.config_dir();
        let config = config_dir.join(PathBuf::from("player-rs/config.ron"));

        if config.exists() {
            Ok(Some(Config::from_path(config)?))
        } else {
            Ok(None)
        }
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotADirectory,
            "couldn't find config directory",
        ))
    }
}

fn save_config(dir: &PathBuf) -> Result<Config> {
    //thinking out loud, this function should probably be a Result because i can't expect it to
    //work. Err on if config dir cant be found, ? functions that should return Result because they
    //SHOULD work. where should i call this function tho?
    if let Some(base) = BaseDirs::new() {
        let config_dir = base.config_dir();
        let config = config_dir.join(PathBuf::from("player-rs/config.ron"));
        create_dir_all(&*config)?;

        let text = to_string_pretty(dir, PrettyConfig::default()).unwrap();

        write(&config, text)?;
        Ok(Config::from_path(config)?)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotADirectory,
            "couldn't find config directory",
        ))
    }
}

impl Setup {
    pub fn new() -> Self {
        //let dir: PathBuf = load_dir().unwrap();
        //let bufs: Vec<PathBuf> = walk(&dir);
        //let tracks: Vec<Track> = vec![];
        Self {
            first: true,
            exit: false,
            dir: None,
            bufs: Vec::new(),
            //tracks,
            input: String::new(),
            status: Status::CheckingDir {
                text: String::new(),
            }, //String::new(),
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<Config>> {
        let config = load_config()?;
        if let Some(config) = config {
            return Ok(Some(config));
        } else {
            while !self.exit {
                terminal.draw(|frame| self.render(frame))?;

                self.handle_events()?;
            }
            Ok(self.dir)
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc => self.exit(),
                KeyCode::Char(char) => self.input.push(char),
                KeyCode::Enter => {
                    if let Status::Success { .. } = self.status {
                        self.exit();
                    } else {
                        self.send()?
                    }
                }
                KeyCode::Backspace => {
                    self.input.pop();
                }
                _ => (),
            },
            _ => (),
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let dir_text: String = match self.dir.as_ref() {
            Some(dir) => dir.to_string_lossy().to_string(),
            None => "none".to_owned(),
        };
        let area: Rect = frame.area();
        //if let Status::Success { text } = self.status {}
        let block: Block = match self.status {
            Status::CheckingDir { ref text } => Block::bordered()
                .title_bottom(
                    "please input a directory. starts in home. paths are absolute, be careful!",
                )
                .title(format!("using dir {} ", text)),
            Status::Confirm { ref text, dir: _ } => Block::bordered()
                .title_bottom("confirm? y/n")
                .title(format!("{}", text)),
            Status::Failed { text: _ } => Block::bordered()
                .title_bottom("failed!")
                .title(format!("bad input?")),
            Status::Success { ref text } => Block::bordered()
                .title_bottom("success!! press enter to continue. your input will be saved")
                .title(format!("{}", text)),
        };
        let msg_area: Rect = Rect {
            x: area.width / 4,
            y: area.height / 4,
            width: area.width / 2,
            height: area.height / 2,
        };
        let text: Paragraph = Paragraph::new(&*self.input).block(block);
        frame.render_widget(text, msg_area);
        //if let Some(text) = &self.dir {
        //    let text: Paragraph = Paragraph::new(text.to_string_lossy().into_owned());
        //    frame.render_widget(text, msg_area);
        //}
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn send(&mut self) -> Result<()> {
        //self.status = "checking".to_owned();

        if let Status::Confirm { text: _, dir } = &self.status {
            let dir: PathBuf = dir.clone(); //was required here, im sorry!!!! needed ownership to
            //dir and i think this is cheap

            match self.input.to_ascii_lowercase().as_str() {
                "y" => {
                    self.status = Status::Success {
                        text: format!("using dir {}", dir.display()),
                    }
                }
                "n" => {
                    self.status = Status::CheckingDir {
                        text: String::new(),
                    }
                }
                _ => {
                    self.status = Status::Confirm {
                        text: "?".to_owned(),
                        dir,
                    }
                }
            }
            self.input.clear();
        } else {
            if let Some(path) = home_dir() {
                self.status = Status::CheckingDir {
                    text: String::from("hi"), //path.to_string_lossy().to_string(),
                };
                let path: PathBuf = path.join(&*self.input);
                self.status = Status::CheckingDir {
                    text: format!("{}", path.to_string_lossy()),
                };
                if path.is_dir() && path.exists() {
                    let (count, big) = probe_dir(&path);
                    if big {
                        self.status = Status::Confirm {
                            text: format!(
                                "that's a big directory, you sure? walking caps at 20k files"
                            ),
                            dir: path.clone(),
                        };
                        self.input.clear();
                        return Ok(());
                        //format!("that's a big directory, {} you sure?", count).to_string();
                    }
                    //self.dir = Some(path);
                    self.status = Status::Confirm {
                        text: format!("found {} files in {}", count, path.display()),
                        dir: path,
                    };
                } else {
                    self.input.clear();
                    self.status = Status::Failed {
                        text: "bad input!".to_owned(),
                    }
                    //self.status = "bad input!".to_owned();
                    //self.dir = Some(path);
                }
            }
            self.input.clear()
        }
        Ok(())
    }
}
