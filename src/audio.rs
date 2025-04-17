use std::{f64::consts::PI, sync::{Arc, Mutex}};

use rand::{rng, Rng};
use sdl3::audio::AudioCallback;

#[derive(Clone, Copy, Debug)]
pub struct AudioData {
    pub white_noise: f32,
}

#[derive(Clone, Debug)]
pub struct AudioHandler {
    data: Arc<Mutex<AudioData>>,
    buffer: Vec<f32>,
    samples: usize,
    sample: usize,
}
impl AudioHandler {
    pub fn new(data: Arc<Mutex<AudioData>>, samples: usize) -> Self {
        Self { data, buffer: Vec::with_capacity(128), samples, sample: 0 }
    }
}
impl AudioCallback<f32> for AudioHandler {
    fn callback(&mut self, stream: &mut sdl3::audio::AudioStream, requested: i32) {
        let mut t = self.sample as f64 / self.samples as f64;
        let mut rng = rng();
        let data_lock = self.data.lock().unwrap();
        let data = *data_lock;
        drop(data_lock);
        self.buffer.clear();
        for _ in 0..(requested as usize) {
            t += 1.0 / self.samples as f64;
            let white_noise = rng.random_range(-1.0..=1.0) * 0.5 * data.white_noise;
            let middle_c = (Wave::new(WaveType::Saw, 440.0, 0.5).get(t) + Wave::new(WaveType::Sine, 440.0, 0.5).get(t) + Wave::new(WaveType::Square, 440.0, 0.5).get(t)) as f32 * data.white_noise;
            self.buffer.push(white_noise + middle_c);
        }

        stream.put_data_f32(&self.buffer).unwrap();
        self.sample += requested as usize;
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wave {
    ty: WaveType,
    hz: f64,
    volume: f64,
}
impl Wave {
    pub fn get(&self, t: f64) -> f64 {
        self.ty.get(self.hz, t) * self.volume
    }
    pub fn new(ty: WaveType, hz: f64, volume: f64) -> Self {
        Self { ty, hz, volume }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaveType {
    Sine,
    Square,
    Saw
}
impl WaveType {
    fn get(&self, hz: f64, t: f64) -> f64 {
        match self {
            WaveType::Sine => (hz * t * 2.0 * PI).sin(),
            WaveType::Square => {
                let scaled_t = (hz * t).floor();
                if scaled_t % 2.0 == 0.0 {
                    -1.0
                } else {
                    1.0
                }
            },
            WaveType::Saw => {
                let scaled_t = (2.0 * hz * t) % 2.0;
                scaled_t - 1.0
            }
        }
    }
}