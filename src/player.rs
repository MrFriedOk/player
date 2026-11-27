use image::ImageReader;
use image::load_from_memory;
use ratatui::layout::Size;
use ratatui_image::Image;
use ratatui_image::Resize;
use ratatui_image::picker;
use ratatui_image::picker::Picker;
use rodio::Decoder;
use rodio::DeviceSinkBuilder;
use rodio::MixerDeviceSink;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

use crate::track::Track;

pub struct Player {
    //track: &Path,
    pub player: rodio::Player,
    pub handle: MixerDeviceSink,
    //track: Option<Track>,
    pub image: Option<Vec<u8>>,
}

impl Player {
    pub fn new() -> Self {
        let mut handle = DeviceSinkBuilder::open_default_sink().expect("j");
        let player = rodio::Player::connect_new(&handle.mixer());
        handle.log_on_drop(false); //ig i have to do it this way?

        Self {
            player,
            handle,
            //track: None,
            image: None,
            //queue: Vec<Track>
        }
    }

    pub fn record(&self) -> Result<()> {
        Ok(())
    }

    pub fn append(&mut self, track: &mut Track) -> Result<(String, PathBuf)> {
        let (label, path_buf) = (track.label, track.path_buf);
        let file = File::open(track.path_buf)?;
        let source = self.player.append(source);
        Ok((label, path_buf))
    }

    pub fn play(&mut self, track: &mut Track) -> Result<()> {
        if let Some(pic) = &track.get_pic() {
            //match image.resize_to_fill
            self.image = Some(pic.to_owned());
        };
        let file = File::open(&track.path_buf)?;
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
