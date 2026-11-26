use image::load_from_memory;
use ratatui_image::Image;
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
    track: Option<None>,
    image: Option<Image>,
}

impl Player {
    pub fn new() -> Self {
        let mut handle = DeviceSinkBuilder::open_default_sink().expect("j");
        let player = rodio::Player::connect_new(&handle.mixer());
        handle.log_on_drop(false); //ig i have to do it this way?

        Self {
            player,
            handle,
            track: None,
            image: None,
        }
    }
    pub fn play(&mut self, track: &Track) -> Result<()> {
        let Some(pic) = track.get_pic() else {
            return Ok(());
        };
        let image = load_from_memory(&pic);
        let file = File::open(track.path_buf)?;
        self.player = rodio::play(&self.handle.mixer(), file).unwrap();
        Ok(())
    }
    pub fn toggle(&self) {
        if self.player.is_paused() {
            self.player.play()
        } else {
            self.player.pause()
        }
    }
}
