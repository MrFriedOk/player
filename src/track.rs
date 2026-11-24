use lofty::file::TaggedFileExt;

use lofty::picture::Picture;
use lofty::picture::error::PictureParseError;
use lofty::{
    file::{FileType, TaggedFile},
    read_from_path,
    tag::{Accessor, Tag},
};

use std::{
    fmt::{Debug, Formatter},
    path::PathBuf,
};

pub struct Track {
    file_name: String,
    file_type: FileType,
    path_buf: PathBuf,
    haystack: String,
    tag: Tag,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub picture: Option<Picture>,
}

impl AsRef<str> for Track {
    fn as_ref(&self) -> &str {
        &self.haystack
    }
}

impl Debug for Track {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Track")
            .field("artist", &self.artist)
            .finish()
    }
}

impl Track {
    pub fn from_path(path: &PathBuf) -> Option<Self> {
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
            picture: None, //i dont think it's a good idea to load every picture of every song on startup
        })
    }

    pub fn get_pic(&mut self) -> Result<Picture, PictureParseError> {
        let picture: Picture = self.tag.pictures();
    }
}
