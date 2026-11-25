use color_eyre::Result;

use directories::UserDirs;

use std::path::PathBuf;
use walkdir::WalkDir;

pub mod player;
pub mod search;
pub mod setup;
pub mod track;
pub mod tui;
use setup::Setup;
use track::Track;
use tui::App;

fn load_dir() -> Result<PathBuf, String> {
    let user_dir: UserDirs = UserDirs::new().ok_or("couldn't find home dir?")?;
    let audio_dir = user_dir
        .audio_dir()
        .ok_or("couldn't find default audio dir?")?;
    Ok(audio_dir.to_path_buf())
}

fn walk(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        //println!("{:?}", entry.path())
        files.push(entry.into_path());
    }
    Ok(files)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let tracks: Vec<Track> = ratatui::run(|terminal| Setup::new().run(terminal))?;

    //let dir: PathBuf = load_dir().unwrap();
    //let bufs: Vec<PathBuf> = walk(&dir);
    //let mut tracks: Vec<Track> = Vec::new();
    //for buf in bufs {
    //    if let Some(track) = Track::from_path(&buf) {
    //        tracks.push(track);
    //    }
    //}
    ratatui::run(|terminal| App::new(tracks).run(terminal))?;

    Ok(())

    //let dir: PathBuf = load_dir().unwrap();
    //let mut tracks: Vec<Track> = Vec::new();
    //println!("default audio directory, {:?}", dir);
    //let files: Vec<PathBuf> = walk(&dir);
    //for file in &files {
    //    if let Some(track) = Track::from_path(file) {
    //        tracks.push(track);
    //    }
    //}
}
