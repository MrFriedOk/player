use directories::{BaseDirs, UserDirs};
use nucleo::*;
use ratatui::*;
use rodio::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn load_dir() -> Result<PathBuf, String> {
    //let mut path: Path = Path::new();

    if let Some(user_dir) = UserDirs::new() {
        let path: &Path = user_dir
            .audio_dir()
            .expect("should have found the audio directory");
        Ok(path.to_path_buf())
    } else {
        Err("couldn't find default audio directory".to_owned())
    }
}

fn main() {
    let dir: PathBuf = load_dir().unwrap();
    println!("{:?}", dir);
}
