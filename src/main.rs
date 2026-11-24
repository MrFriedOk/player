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

use lofty::file::TaggedFileExt;

use lofty::{
    file::{FileType, TaggedFile},
    read_from_path,
    tag::{Accessor, Tag, TagType},
};

struct Track {
    file_name: String,
    file_type: FileType,
    path_buf: PathBuf,
    haystack: String,
    tag: Tag,
    pub artist: String,
    pub title: String,
    pub album: String,
}

impl AsRef<str> for Track {
    fn as_ref(&self) -> &str {
        &self.haystack
    }
}

impl Track {
    fn from_path(path: &PathBuf) -> Option<Self> {
        let path_buf: PathBuf = path.to_owned();
        let file_type: FileType = FileType::from_path(path)?;
        let file_name: String = path.file_name()?.to_string_lossy().into_owned();
        let tagged: TaggedFile = read_from_path(path).ok()?;
        let tag: Tag = tagged.primary_tag()?.to_owned();
        let artist: String = tag.artist()?.into_owned();
        let title: String = tag.title()?.into_owned();
        let album: String = tag.album()?.into_owned();
        //let thing = tag.tag_type();
        Some(Self {
            file_name: file_name.clone(),
            file_type,
            path_buf,
            haystack: format!(
                "{} {:?} {:?} {:?} {:?}",
                file_name, file_type, artist, title, album,
            ),
            tag: tag.clone(),
            artist,
            title,
            album,
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

fn search(input: String, tracks: Vec<Track>) {
    //let input: String = input.to_lowercase();
    for track in &tracks {
        println!("{:?}", track.haystack);
    }
    let mut matcher: Matcher = Matcher::new(Config::DEFAULT.match_paths());
    let matches = Pattern::parse(&input, CaseMatching::Ignore, Normalization::Smart)
        .match_list(tracks, &mut matcher);

    for match2 in matches {
        println!("{:?}, score: {}", match2.0.artist, match2.1);
    }
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
    search("jojo's bizarre".to_owned(), tracks)
    //scan(&dir);
}
