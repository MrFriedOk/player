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

#[derive(Clone)]
pub struct Track {
    file_name: String,
    file_type: FileType,
    pub path_buf: PathBuf,
    pub haystack: String,
    //tag: Tag,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
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
        let tag = tagged.primary_tag().or_else(|| tagged.first_tag()); //fallback, idk
        //if this is worth it cuz what if first_tag isn't  athing?
        //
        //trying out chained functions more, and_then returns an Option of my own defining(?).
        //basically like using `let Some() else {return None}` which is the type of thing i do a lot.
        //also didn't realize this but stuff like `into` functions, those consume the original value
        //and produce... whatever the output is, saving LOTS of memory here.
        let artist = tag.and_then(|tag| tag.artist().map(|string| string.into_owned())); //String::new();
        let title = tag.and_then(|tag| tag.title().map(|string| string.into_owned()));
        let album = tag.and_then(|tag| tag.album().map(|string| string.into_owned()));
        //let tag = tagged.primary_tag().or_else(|| tagged.first_tag());
        //let artist = tag.artist()?.into_owned();
        //let title = tag.title()?.into_owned();
        //let album = tag.album()?.into_owned();

        //let thing = tag.tag_type();
        Some(Self {
            file_name: file_name.clone(),
            file_type,
            path_buf,
            haystack: format!(
                "{} {:?} {:?} {:?} {:?}",
                file_name, file_type, artist, title, album,
            ),
            //tag,
            artist,
            title,
            album,
            picture: None, //i dont think it's a good idea to load every picture of every song on startup
        })
    }

    pub fn get_pic(&mut self) -> Option<Vec<u8>> {
        let tagged = read_from_path(&self.path_buf).ok()?;
        let tag = tagged.primary_tag().or_else(|| tagged.first_tag())?;
        let pic = tag
            .pictures()
            .iter()
            .find(|pic| pic.pic_type() == PictureType::CoverFront);

        match pic {
            Some(pic) => return Some(pic.data().to_vec()),
            None => return None,
        }
        // => return self.picture = Some(pic.data().to_vec()),
    }
}
