use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::io::Result;
use std::path::PathBuf;

fn player(buf: PathBuf) -> Result<()> {
    let mut handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::Player::connect_new(handle.mixer());
    handle.log_on_drop(false);

    let buf = BufReader::new(File::open(buf).unwrap());

    let source = Decoder::try_from(buf).unwrap();

    player.append(source);

    //player.sleep_until_end(); this breaks everything
    Ok(())
}
