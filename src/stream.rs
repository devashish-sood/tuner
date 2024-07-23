use std::sync::{Arc, RwLock};
use cpal::{
    traits::{DeviceTrait, HostTrait}, Device, StreamConfig
};

use crate::fsvec::FixedSizeVec;

pub fn build_stream(buf: Arc<RwLock<FixedSizeVec<f32>>>, device: Device, config: StreamConfig) -> cpal::Stream {
    device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let buffer = buf.write();
                match buffer {
                    Ok(mut buffer) => {
                        buffer.extend(data);
                    }
                    Err(_) => {
                    }
                }
                if let Ok(mut buffer) = buf.write() {
                    buffer.extend(data);
                }
            },
            move |err| print!("error: {}", err),
            None,
        )
        .expect("Could not build stream")
}

pub fn get_device() -> Device {
    cpal::default_host()
    .default_input_device()
    .expect("No default input device found")
}

pub fn get_config(device:&Device) -> StreamConfig {
    device
    .default_input_config()
    .expect("No default input config found")
    .into()
}