use crate::error::{Result, WaveformError};

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub name: String,
}

impl Unit {
    pub fn seconds() -> Self {
        Self {
            name: "s".to_string(),
        }
    }

    pub fn volts() -> Self {
        Self {
            name: "V".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Channel {
    pub name: String,
    pub unit: Unit,
    pub samples: Vec<f64>,
}

impl Channel {
    pub fn new(name: impl Into<String>, unit: Unit, samples: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            unit,
            samples,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Waveform {
    pub time_unit: Unit,
    pub time: Vec<f64>,
    pub channels: Vec<Channel>,
}

impl Waveform {
    pub fn new(time: Vec<f64>, channels: Vec<Channel>) -> Result<Self> {
        if time.is_empty() {
            return Err(WaveformError::EmptyInput);
        }
        for channel in &channels {
            if channel.samples.len() != time.len() {
                return Err(WaveformError::MismatchedSampleCount {
                    expected: time.len(),
                    actual: channel.samples.len(),
                });
            }
        }
        Ok(Self {
            time_unit: Unit::seconds(),
            time,
            channels,
        })
    }

    pub fn sample_count(&self) -> usize {
        self.time.len()
    }

    pub fn channel(&self, name: &str) -> Option<&Channel> {
        self.channels.iter().find(|channel| channel.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_aligned_channel_lengths() {
        let waveform = Waveform::new(
            vec![0.0, 0.1],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0, 5.0])],
        )
        .expect("waveform should be valid");

        assert_eq!(waveform.sample_count(), 2);
    }

    #[test]
    fn rejects_mismatched_channel_lengths() {
        let result = Waveform::new(
            vec![0.0, 0.1],
            vec![Channel::new("input_v", Unit::volts(), vec![0.0])],
        );

        assert_eq!(
            result,
            Err(WaveformError::MismatchedSampleCount {
                expected: 2,
                actual: 1
            })
        );
    }
}
