use std::{time::Duration, sync::{Arc, atomic::{AtomicBool, Ordering}}};

use rodio::{Source, source::SineWave};

struct ConditionalSineWave {
    sine_wave: SineWave,
    condition: Arc<AtomicBool>,
}

impl ConditionalSineWave {
    pub fn new(freq: f32, condition: Arc<AtomicBool>) -> Self {
        ConditionalSineWave {
            sine_wave: SineWave::new(freq),
            condition,
        }
    }
}

impl Source for ConditionalSineWave {
    fn current_frame_len(&self) -> Option<usize> {
        self.sine_wave.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.sine_wave.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.sine_wave.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.sine_wave.total_duration()
    }    
}

impl Iterator for ConditionalSineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.condition.load(Ordering::Relaxed) {
            self.sine_wave.next()
        } else {
            Some(0.0)
        }
    }
}

// TODO : Fix underrun problem
pub fn play_beep(condition: Arc<AtomicBool>) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    sink.append(ConditionalSineWave::new(440.0, condition));
    sink.sleep_until_end();
}