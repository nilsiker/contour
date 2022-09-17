use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn bit_font_bytes() -> Vec<u8> {
    let work_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => panic!("Could not access program working directory."),
    };
    let path = work_dir.join("assets/fonts/PressStart2P.ttf");
    let file = File::open(path).expect("Could not find target file.");
    let mut reader = BufReader::new(file);
    let mut buf = Vec::new();

    reader.read_to_end(&mut buf).unwrap();

    buf
}

pub mod paths {
    pub const AUDIO_BGM: &str = "audio/bgm.wav";
    pub const SPRITE_DARKNESS: &str = "sprites/darkness.png";
    pub const SPRITE_PLAYER: &str = "sprites/player.png";
    pub const SPRITE_ENEMY: &str = "sprites/enemy.png";
}
