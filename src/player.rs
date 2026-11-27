use color_eyre::Result;
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
use std::path::Path;
use std::path::PathBuf;

use crate::track::Track;

pub struct Queue {
    pub items: Vec<Track>,
    pub current: Option<usize>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current: None,
        }
    }

    pub fn append_from_track(&mut self, track: &Track) {
        if self.current.is_none() {
            self.current = Some(0);
        }
        let track = track.to_owned();
        self.items.push(track);
    }

    pub fn load(&mut self, player: &mut Player) -> Result<()> {
        //self.current = Some(1);
        if let Some(current) = self.current {
            player.play(&mut self.items[current])?;
        }
        Ok(())
    }
}

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

    pub fn update(&self, queue: &Queue) {
        if self.player.empty()
            && let Some(current) = queue.current
        {
            self.play(&mut queue.items[current]);
        }
    }

    pub fn append(&mut self, track: &mut Track) -> Result<(String, PathBuf)> {
        let (label, path_buf) = (track.label.to_owned(), track.path_buf.to_owned());
        let file = File::open(&path_buf)?;
        let source = Decoder::try_from(file)?;
        self.player.append(source);
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
