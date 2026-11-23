use directories::{BaseDirs, UserDirs};
use nucleo::*;
use ratatui::*;
use rodio::*;
use std::{fs, path::Path};

fn load_dir() -> Result<&Path, String> {
    //let mut path: Path = Path::new();

    if let Some(path) = UserDirs::audio_dir() {
        Ok(path)
    } else {
        Err("couldn't find default audio directory".to_owned())
    }
}

fn main() {}
