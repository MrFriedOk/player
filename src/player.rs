use rodio::Decoder;
use rodio::DeviceSinkBuilder;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

use crate::track::Track;

pub struct Player {
    //track: &Path,
}

impl Player {
    pub fn new() -> Self {
        let mut handle = DeviceSinkBuilder::open_default_sink().expect("open defaul audio stream");
        let player = rodio::Player::connect_new(&handle.mixer());
        handle.log_on_drop(false); //ig i have to do it this way?

        Self {}
    }
    pub fn play(&self) {
        let mut handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(handle.mixer());
        handle.log_on_drop(false);

        let buf = BufReader::new(File::open(&self.track.path_buf).unwrap());

        let source = Decoder::try_from(buf).unwrap();

        player.append(source);

        //player.sleep_until_end(); //this breaks everything
    }
}
