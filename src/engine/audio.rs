use sdl2::mixer::{Chunk, Music};
use std::collections::HashMap;

pub struct AudioSystem<'a> {
    sfx: HashMap<String, Chunk>,
    bgm: Option<Music<'a>>, 
}

impl<'a> AudioSystem<'a> {
    pub fn new() -> Self {
        sdl2::mixer::init(sdl2::mixer::InitFlag::MP3).unwrap(); 
        sdl2::mixer::open_audio(44100, sdl2::mixer::AUDIO_S16LSB, 2, 2048).unwrap();
        sdl2::mixer::allocate_channels(16);
        Self { sfx: HashMap::new(), bgm: None }
    }

    pub fn load_sfx(&mut self, name: &str, path: &str) {
        if let Ok(chunk) = Chunk::from_file(path) { 
            self.sfx.insert(name.to_string(), chunk); 
        }
    }

    pub fn play_sfx(&self, name: &str) {
        if let Some(chunk) = self.sfx.get(name) { 
            let _ = sdl2::mixer::Channel::all().play(chunk, 0); 
        }
    }

    pub fn play_bgm(&mut self, path: &str) {
        if let Ok(music) = Music::from_file(path) {
            let _ = music.play(-1);
            self.bgm = Some(music);
        }
    }
}