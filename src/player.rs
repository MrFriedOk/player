use rodio::Decoder;
use rodio::DeviceSinkBuilder;
use rodio::MixerDeviceSink;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

use crate::track::Track;

pub struct Player {
    //track: &Path,
    player: rodio::Player,
    handle: MixerDeviceSink,
}

impl Player {
    pub fn new() -> Self {
        let mut handle = DeviceSinkBuilder::open_default_sink().expect("j");
        let player = rodio::Player::connect_new(&handle.mixer());
        handle.log_on_drop(false); //ig i have to do it this way?

        Self { player, handle }
    }
    pub fn play(&mut self, path: &PathBuf) -> Result<()> {
        let file = File::open(path)?;
        self.player = rodio::play(&self.handle.mixer(), file).unwrap();
        Ok(())
    }
}
