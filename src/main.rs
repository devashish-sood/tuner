use std::{io::{self, Write}, sync::{Arc, RwLock}};
use cpal::traits::StreamTrait;
use fsvec::FixedSizeVec;
use pitch_detection::detector::{yin::YINDetector, PitchDetector};
use stream::{get_config, get_device};
mod stream;
mod fsvec;

const A4: f32 = 440.0;
const NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

fn get_note(freq: f32) -> String {
    let base = A4 * (2 as f32).powf(-4.734);
    let steps = ((freq / base).log2() * 12_f32).round() as usize;
    format!("{}{}", NOTES[steps % 12], steps / 12)
}

fn main() {
    const BUF_SIZE: usize = 1024;
    const POWER_THRESHOLD: f32 = 1.0;
    const CLARITY_THRESHOLD: f32 = 0.25;
    let mut stdout = io::stdout();
    let device = get_device();
    let config = get_config(&device);
    let sample_rate = config.sample_rate.0 as usize;
    let buf= Arc::new(RwLock::new(FixedSizeVec::<f32>::new(BUF_SIZE)));
    let mut detector = YINDetector::<f32>::new(BUF_SIZE, BUF_SIZE / 2);
    let stream: cpal::Stream = stream::build_stream(buf.clone(), device, config);
    stream.play().expect("Stream couldn't play");
    loop {
        let buffer = buf.read();
        match buffer {
            Ok(buffer) => {
                if buffer.full() {
                    let sample = buffer.as_slice();
                    let pitch = detector.get_pitch(sample, sample_rate as usize, POWER_THRESHOLD, CLARITY_THRESHOLD);
                    match pitch {
                        Some(pitch) => {
                            print!("\r{} ", get_note(pitch.frequency));
                            stdout.flush().unwrap();
                        },
                        None => {
                            //Nothing to be printed
                        }
                    }
                }
            }
            Err(_) => {
                // No operation needed
            }
        }
    }
}
