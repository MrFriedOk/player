use color_eyre::Result;

use directories::UserDirs;
use nucleo_matcher::{
    Config, Matcher,
    pattern::{CaseMatching, Normalization, Pattern},
};

use rodio::{
    Decoder, MixerDeviceSink,
    source::{SineWave, Source},
};
use std::{fs::File, path::PathBuf, time::Duration};
use walkdir::WalkDir;

pub mod track;
pub mod tui;
use track::Track;
use tui::App;

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

fn player(buf: PathBuf) -> Result<()> {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(&handle.mixer());
    handle.log_on_drop(false);

    player.sleep_until_end();
    Ok(())
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::default().run(terminal));

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
