use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use directories::{BaseDirs, UserDirs};
use nucleo_matcher::{
    Config, Matcher,
    pattern::{CaseMatching, Normalization, Pattern},
};
use ratatui::{
    DefaultTerminal, Frame,
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use walkdir::WalkDir;
//use rodio::*;
use std::path::PathBuf;

pub mod track;
use track::Track;

fn load_dir() -> Result<PathBuf, String> {
    let user_dir: UserDirs = UserDirs::new().ok_or("couldn't find home dir?")?;
    let audio_dir = user_dir
        .audio_dir()
        .ok_or("couldn't find default audio dir?")?;
    Ok(audio_dir.to_path_buf())
}

fn walk(dir: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        //println!("{:?}", entry.path())
        files.push(entry.into_path());
    }
    files
}

fn search(input: String, tracks: Vec<Track>) -> Vec<(Track, u32)> {
    //let input: String = input.to_lowercase();
    //or track in &tracks {
    //    println!("{:?}", track.haystack);
    //}

    //create a new matcher with default config, see the nucleo docs for default config.
    //next, define matches, which is a Vec of the Track and the score the Track got. requires
    //AsRef<str>, implemented in my Track object and simply returns the Track's haystack as a str.
    let mut matcher: Matcher = Matcher::new(Config::DEFAULT.match_paths());
    let matches = Pattern::parse(&input, CaseMatching::Ignore, Normalization::Smart)
        .match_list(tracks, &mut matcher);

    matches
}

fn main() -> Result<()> {
    let dir: PathBuf = load_dir().unwrap();
    let mut tracks: Vec<Track> = Vec::new();
    println!("default audio directory, {:?}", dir);
    let files: Vec<PathBuf> = walk(&dir);
    for file in &files {
        if let Some(track) = Track::from_path(file) {
            tracks.push(track);
        }
    }
    color_eye::install()?;
    let terminal: Terminal<CrosstermBackend> = ratatui::init();
}
