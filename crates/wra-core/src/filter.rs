use crate::error::{Result, WaveformError};
use crate::model::{Channel, Waveform};

const TAU: f64 = std::f64::consts::PI * 2.0;

pub trait Filter {
    fn name(&self) -> &'static str;
    fn apply(&self, waveform: &Waveform) -> Result<Waveform>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MovingAverageFilter {
    pub window_samples: usize,
}

impl Filter for MovingAverageFilter {
    fn name(&self) -> &'static str {
        "moving_average"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        if self.window_samples == 0 {
            return Err(WaveformError::InvalidParameter {
                name: "window_samples".to_string(),
                reason: "must be greater than zero".to_string(),
            });
        }

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples = trailing_moving_average(&channel.samples, self.window_samples);
                Channel::new(channel.name.clone(), channel.unit.clone(), samples)
            })
            .collect();

        Waveform::new(waveform.time.clone(), channels)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LowPassFilter {
    pub cutoff_hz: f64,
}

impl Filter for LowPassFilter {
    fn name(&self) -> &'static str {
        "low_pass"
    }

    fn apply(&self, waveform: &Waveform) -> Result<Waveform> {
        if self.cutoff_hz <= 0.0 {
            return Err(WaveformError::InvalidParameter {
                name: "cutoff_hz".to_string(),
                reason: "must be greater than zero".to_string(),
            });
        }
        validate_time_axis(&waveform.time)?;

        let channels = waveform
            .channels
            .iter()
            .map(|channel| {
                let samples =
                    first_order_low_pass(&waveform.time, &channel.samples, self.cutoff_hz);
                Channel::new(channel.name.clone(), channel.unit.clone(), samples)
            })
            .collect();

        Waveform::new(waveform.time.clone(), channels)
    }
}

fn trailing_moving_average(samples: &[f64], window_samples: usize) -> Vec<f64> {
    let mut filtered = Vec::with_capacity(samples.len());
    for index in 0..samples.len() {
        let start = (index + 1).saturating_sub(window_samples);
        let window = &samples[start..=index];
        filtered.push(window.iter().sum::<f64>() / window.len() as f64);
    }
    filtered
}

fn validate_time_axis(time: &[f64]) -> Result<()> {
    for pair in time.windows(2) {
        if pair[1] <= pair[0] {
            return Err(WaveformError::InvalidWaveform {
                reason: "time samples must be strictly increasing for low-pass filtering"
                    .to_string(),
            });
        }
    }
    Ok(())
}

fn first_order_low_pass(time: &[f64], samples: &[f64], cutoff_hz: f64) -> Vec<f64> {
    if samples.is_empty() {
        return Vec::new();
    }

    let rc = 1.0 / (TAU * cutoff_hz);
    let mut filtered = Vec::with_capacity(samples.len());
    filtered.push(samples[0]);

    for index in 1..samples.len() {
        let dt = time[index] - time[index - 1];
        let alpha = dt / (rc + dt);
        let previous = filtered[index - 1];
        filtered.push(previous + alpha * (samples[index] - previous));
    }

    filtered
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Unit;

    fn waveform(samples: Vec<f64>) -> Waveform {
        Waveform::new(
            vec![0.0, 1.0, 2.0, 3.0],
            vec![Channel::new("input_v", Unit::volts(), samples)],
        )
        .expect("test waveform should be valid")
    }

    #[test]
    fn moving_average_filters_each_channel_without_mutating_input() {
        let input = waveform(vec![0.0, 2.0, 4.0, 6.0]);
        let filtered = MovingAverageFilter { window_samples: 2 }
            .apply(&input)
            .expect("filter should apply");

        assert_eq!(input.channels[0].samples, vec![0.0, 2.0, 4.0, 6.0]);
        assert_eq!(filtered.channels[0].samples, vec![0.0, 1.0, 3.0, 5.0]);
    }

    #[test]
    fn moving_average_rejects_zero_window() {
        let result =
            MovingAverageFilter { window_samples: 0 }.apply(&waveform(vec![0.0, 1.0, 2.0, 3.0]));

        assert!(matches!(
            result,
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn low_pass_smooths_step_input() {
        let input = waveform(vec![0.0, 10.0, 10.0, 10.0]);
        let filtered = LowPassFilter { cutoff_hz: 0.1 }
            .apply(&input)
            .expect("filter should apply");

        assert_eq!(filtered.channels[0].samples[0], 0.0);
        assert!(filtered.channels[0].samples[1] > 0.0);
        assert!(filtered.channels[0].samples[1] < 10.0);
        assert!(filtered.channels[0].samples[3] < 10.0);
    }
}
