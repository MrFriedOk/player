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

fn load_dir() -> Result<PathBuf, String> {
    let user_dir: UserDirs = UserDirs::new().ok_or("couldn't find home dir?")?;
    let audio_dir = user_dir
        .audio_dir()
        .ok_or("couldn't find default audio dir?")?;
    Ok(audio_dir.to_path_buf())
}

fn scan(dirs: String) {
    todo!()
}

fn walk(dir: &PathBuf) {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        println!("{:?}", entry.path())
    }
}

fn main() {
    let dir: PathBuf = load_dir().unwrap();
    println!("default audio directory, {:?}", dir);
    walk(&dir);
    //scan(&dir);
}
