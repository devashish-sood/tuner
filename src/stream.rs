use cpal::{
    traits::{DeviceTrait, HostTrait},
    StreamConfig,
};

use crate::{detection::apply_fft, fsvec::FixedSizeVec};

pub fn build_stream() -> cpal::Stream {
    let host = cpal::default_host();
    let my_dev = host
        .default_input_device()
        .expect("no default input device found");
    let config: StreamConfig = StreamConfig::from(my_dev
        .default_input_config()
        .expect("No default input config"));
    let sample_rate = config.sample_rate.0 as f32;
    let mut acc= FixedSizeVec::<f32>::new(sample_rate as usize);
    let stream = my_dev
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                acc.extend(data); 
                if acc.full() {
                    apply_fft(acc.as_slice());
                }
            },
            move |err| print!("error: {}", err),
            None,
        )
        .expect("Could not build stream");

    stream
}
