use rustfft::{num_complex::Complex, FftPlanner};

pub fn apply_fft(sample: &[f32]) {
  let mut datapoints = sample.iter().map(|&x| Complex::new(x, 0.0)).collect::<Vec<_>>();
  let sample_rate = datapoints.len();
  let mut planner = FftPlanner::<f32>::new();
  let fft = planner.plan_fft_forward(sample_rate);
  fft.process(&mut datapoints);
  construct_hps(&mut datapoints, sample_rate);
}

fn construct_hps(datapoints: &[Complex<f32>], sample_rate: usize) {
  let mut hps = vec![0.0; datapoints.len() / 2];
  for harmonic in 1..=4 {
      for i in 0..hps.len() {
          if i * harmonic < datapoints.len() / 2 {
              hps[i] += datapoints[i * harmonic].norm();
          }
      }
    }

    let mut freq_mag_pairs: Vec<(f32, f32)> = hps.iter().enumerate()
    .map(|(i, &mag)| {
        let freq = i as f32 * sample_rate as f32 / datapoints.len() as f32;
        (freq, mag)
    })
    .collect();

  freq_mag_pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (frequency, magnitude) in freq_mag_pairs.into_iter().take(10) {
        println!("Frequency: {}, Magnitude: {}", frequency, magnitude);
    }
  }