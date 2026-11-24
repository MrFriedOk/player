use crate::track::Track;

pub struct Setup {
    first: bool,
    dir: PathBuf,
    bufs: Vec<PathBuf>,
    tracks: Vec<Track>,
}
