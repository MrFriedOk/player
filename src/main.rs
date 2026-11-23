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

struct Track {
    file_name: String,
    file_type: FileType,
    path_buf: PathBuf,
}

impl Track {
    fn from_path(path: &PathBuf) -> Option<Self> {
        Some(Self {
            file_name: path.file_name()?.to_string_lossy().into_owned(),
            file_type: FileType::from_path(path)
                .ok_or("couldn't guess file type")
                .unwrap(),
            path_buf: *path,
        })
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

fn search(input: String, files: Vec<PathBuf>) {
    let input: String = input.to_lowercase();
    //let files: AsRef<str> = files.as_ref();
    //let mut matcher: Matcher = Matcher::new(Config::DEFAULT.match_paths());
    //let matches = Pattern::parse(&input, CaseMatching::Ignore, Normalization::Smart)
    //    .match_list(files, &mut matcher);
}

fn main() {
    let dir: PathBuf = load_dir().unwrap();
    println!("default audio directory, {:?}", dir);
    let files: Vec<PathBuf> = walk(&dir);
    for file in files {
        let track: Track = Track::from_path(&file);
        println!("{:?}", track)
    }
    println!("{:?}", files);
    search("the".to_owned(), files)
    //scan(&dir);
}
