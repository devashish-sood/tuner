use std::collections::HashMap;
use rustfft::{num_complex::{Complex, ComplexFloat}, FftPlanner};

pub fn apply_fft(sample: &[f32]) {
  let mut datapoints = sample.iter().map(|&x| Complex::new(x, 0.0)).collect::<Vec<_>>();
  let mut planner = FftPlanner::<f32>::new();
  let fft = planner.plan_fft_forward(datapoints.len());
  fft.process(&mut datapoints);
  
  construct_hps(&mut datapoints, sample.len());

}

fn construct_hps(datapoints: &[Complex<f32>], sample_rate: usize) {
  println!("sample rate: {}, d len: {} ", sample_rate, datapoints.len());
  let mut hps = HashMap::<String, f32>::new();
  let base_vec: Vec<(i32, f32)> = datapoints[..500]
  .iter()
  .enumerate()
  .map(|(i, &c)| {
    let magnitude = c.abs();
    let frequency = (i as i32) * (sample_rate as i32) / (datapoints.len() as i32);
    println!("base freq: {}, with mag: {}", frequency, magnitude);
    for i in 1..=4 {
      let hps_mag = hps.entry((frequency / i).to_string()).or_insert(1.0);
      *hps_mag *= magnitude;
    }
    (frequency, magnitude)})
    .collect();



  let mut vec_hps: Vec<(&String, &f32)> = hps.iter().collect();

  vec_hps.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    // for (frequency, magnitude) in &base_vec {
    //     println!("Frequency: {}, Magnitude: {}", frequency, magnitude);
    // }
    // println!("New Entry\n");
  }