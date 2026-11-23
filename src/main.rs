use directories::{BaseDirs, UserDirs};
use nucleo_matcher::{
    Config, Matcher,
    pattern::{CaseMatching, Normalization, Pattern},
};
use walkdir::WalkDir;
//use ratatui::*;
//use rodio::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

use lofty::file::FileType;

#[derive(Debug)]
struct Track {
    file_name: String,
    file_type: FileType,
    path_buf: PathBuf,
    haystack: String,
}

impl Track {
    fn from_path(path: &PathBuf) -> Option<Self> {
        Some(Self {
            file_name: path.file_name()?.to_string_lossy().into_owned(),
            file_type: FileType::from_path(path)?,
            path_buf: path.to_owned(),
            haystack: "".to_owned(),
        })
    }

    fn gen_haystack(&mut self) {
        let haystack: String = "".to_owned();
    }
}

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

fn search(input: String, tracks: Vec<Track>) {
    let input: String = input.to_lowercase();
    //let mut matcher: Matcher = Matcher::new(Config::DEFAULT.match_paths());
    //let matches = Pattern::parse(&input, CaseMatching::Ignore, Normalization::Smart)
    //    .match_list(files, &mut matcher);
}

fn main() {
    let dir: PathBuf = load_dir().unwrap();
    let mut tracks: Vec<Track> = Vec::new();
    println!("default audio directory, {:?}", dir);
    let files: Vec<PathBuf> = walk(&dir);
    for file in &files {
        if let Some(track) = Track::from_path(file) {
            tracks.push(track);
        }
    }
    //println!("{:?}", &files);
    search("the".to_owned(), tracks)
    //scan(&dir);
}
