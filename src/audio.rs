use std::sync::{Arc, Mutex};

use rand::{rng, Rng};
use sdl3::audio::AudioCallback;

pub struct AudioData {
    pub white_noise: f32,
}

pub struct AudioHandler {
    data: Arc<Mutex<AudioData>>,
    buffer: Vec<f32>,
    samples: usize,
}
impl AudioHandler {
    pub fn new(data: Arc<Mutex<AudioData>>, samples: usize) -> Self {
        Self { data, buffer: Vec::with_capacity(128), samples }
    }
}
impl AudioCallback<f32> for AudioHandler {
    fn callback(&mut self, stream: &mut sdl3::audio::AudioStream, requested: i32) {
        let mut rng = rng();
        let data_lock = self.data.lock().unwrap();
        self.buffer.clear();
        for x in 0..(requested as usize) {
            self.buffer.push(rng.random_range(-1.0..=1.0) * data_lock.white_noise);
        }

        stream.put_data_f32(&self.buffer).unwrap()
    }
}