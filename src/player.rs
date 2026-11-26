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
    player: rodio::Player,
    handle: MixerDeviceSink,
    track: Option<Track>,
    pub image: Option<Image>,
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
        if let Some(pic) = track.get_pic() {
            let image = ImageReader::new(Cursor::new(&pic))
                .with_guessed_format()?
                .decode()
                .unwrap();

            //match image.resize_to_fill
            self.image = Some(Image::new(
                &Picker::halfblocks()
                    .new_protocol(
                        image,
                        Size::new(
                            image.width().div_ceil(24) as u16,
                            image.height().div_ceil(24) as u16,
                        ),
                        Resize::Fit(None),
                    )
                    .unwrap(),
            ));
        };
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
