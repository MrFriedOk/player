use lofty::file::TaggedFileExt;

use std::io::Result;

use lofty::picture::error::PictureParseError;
use lofty::picture::{Picture, PictureType};
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
    pub picture: Option<Vec<u8>>,
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
            tag,
            artist,
            title,
            album,
            picture: None, //i dont think it's a good idea to load every picture of every song on startup
        })
    }

    pub fn get_pic(&mut self) -> Result<()> {
        let pic = self
            .tag
            .pictures()
            .iter()
            .find(|pic| pic.pic_type() == PictureType::CoverFront)?
            .data()
            .to_vec();

        self.picture = Some(pic);
        Ok(())
    }
}
