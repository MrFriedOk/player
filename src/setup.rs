use std::env::home_dir;
use std::fs::canonicalize;
use std::io::Error;
use std::path::PathBuf;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Paragraph;

use crate::probe_dir;
use crate::track::Track;
use crate::{load_dir, walk};
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
                text: "".to_owned(),
            }, //String::new(),
        }
    }
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Vec<Track>> {
        let tracks: Vec<Track> = Vec::new();
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            self.handle_events()?;
        }
        Ok(tracks) //return the Vec once done, make sure to change this lol
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event) if event.kind == KeyEventKind::Press => match event.code {
                KeyCode::Esc => self.exit(),
                KeyCode::Char(char) => self.input.push(char),
                KeyCode::Enter => self.send()?,
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
                .title_bottom("success!!")
                .title(format!("using {}", text)),
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
        match &self.status {
            Status::Confirm { text, dir } => {
                if self.input == "y" {
                    self.dir = Some(dir.to_path_buf());
                    self.status = Status::Success {
                        text: format!("using dir {}", dir.to_string_lossy()),
                    }
                }
                if self.input == "n" {
                    self.status = Status::CheckingDir {
                        text: "".to_owned(),
                    };
                    self.input.clear()
                } else {
                    self.status = Status::Confirm {
                        text: "?".to_owned(),
                        dir: dir.to_path_buf(),
                    };
                    self.input.clear();
                }
            }
            _ => {
                if let Some(path) = home_dir() {
                    let path: PathBuf = path.join(&*self.input);
                    self.status = Status::CheckingDir {
                        text: format!("{}", path.to_string_lossy()),
                    };
                    if path.is_dir() && path.exists() {
                        let (count, big) = probe_dir(&path);
                        if big {
                            self.status = Status::Confirm {
                                text: format!("that's a big directory, you sure? y/n"),
                                dir: path.clone(),
                            };
                            self.input.clear();
                            return Ok(());
                            //format!("that's a big directory, {} you sure?", count).to_string();
                        }
                        //self.dir = Some(path);
                        self.status = Status::Confirm {
                            text: format!("found {} files", count),
                            dir: path,
                        };
                        self.input.clear();
                    } else {
                        self.input.clear();
                        self.status = Status::Failed {
                            text: "bad input!".to_owned(),
                        }
                        //self.status = "bad input!".to_owned();
                    }
                    //self.dir = Some(path);
                }
            }
        }
        Ok(())
    }
}
