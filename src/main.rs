use directories::{BaseDirs, UserDirs};
use nucleo::*;
use ratatui::*;
use rodio::*;
use std::{fs, path::Path};

fn load_dir() -> Result<&'static Path, String> {
    //let mut path: Path = Path::new();

    if let Some(user_dir) = UserDirs::new() {
        let path: &Path = user_dir
            .audio_dir()
            .expect("should have found the audio directory");
        Ok(path)
    } else {
        Err("couldn't find default audio directory".to_owned())
    }
}

fn main() {}
