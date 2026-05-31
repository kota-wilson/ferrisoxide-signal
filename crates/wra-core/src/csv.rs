use crate::error::{Result, WaveformError};
use crate::model::{Channel, Unit, Waveform};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvParseOptions {
    pub delimiter: char,
    pub time_column: String,
    pub channel_columns: Vec<String>,
}

impl CsvParseOptions {
    pub fn new(time_column: impl Into<String>, channel_columns: Vec<String>) -> Self {
        Self {
            delimiter: ',',
            time_column: time_column.into(),
            channel_columns,
        }
    }
}

pub trait WaveformParser {
    fn parse_str(&self, input: &str, options: &CsvParseOptions) -> Result<Waveform>;
}

#[derive(Debug, Default)]
pub struct SimpleCsvParser;

impl WaveformParser for SimpleCsvParser {
    fn parse_str(&self, input: &str, options: &CsvParseOptions) -> Result<Waveform> {
        if input.trim().is_empty() {
            return Err(WaveformError::EmptyInput);
        }

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(delimiter_byte(options.delimiter)?)
            .trim(csv::Trim::All)
            .from_reader(input.as_bytes());

        let header = reader.headers().map_err(|error| WaveformError::Csv {
            message: error.to_string(),
        })?;
        let columns: Vec<&str> = header.iter().collect();
        let time_index = find_column(&columns, &options.time_column)?;
        let channel_indices = options
            .channel_columns
            .iter()
            .map(|column| find_column(&columns, column).map(|index| (column.clone(), index)))
            .collect::<Result<Vec<_>>>()?;

        let mut time = Vec::new();
        let mut channel_samples = vec![Vec::new(); channel_indices.len()];

        for record in reader.records() {
            let record = record.map_err(|error| WaveformError::Csv {
                message: error.to_string(),
            })?;
            time.push(parse_number(record.get(time_index), &options.time_column)?);
            for (sample_index, (column, column_index)) in channel_indices.iter().enumerate() {
                channel_samples[sample_index]
                    .push(parse_number(record.get(*column_index), column)?);
            }
        }

        let channels = channel_indices
            .into_iter()
            .zip(channel_samples)
            .map(|((name, _), samples)| Channel::new(name, Unit::volts(), samples))
            .collect();

        Waveform::new(time, channels)
    }
}

fn delimiter_byte(delimiter: char) -> Result<u8> {
    if delimiter.is_ascii() {
        Ok(delimiter as u8)
    } else {
        Err(WaveformError::InvalidParameter {
            name: "delimiter".to_string(),
            reason: "must be an ASCII character".to_string(),
        })
    }
}

fn find_column(columns: &[&str], target: &str) -> Result<usize> {
    columns
        .iter()
        .position(|column| *column == target)
        .ok_or_else(|| WaveformError::MissingColumn {
            column: target.to_string(),
        })
}

fn parse_number(value: Option<&str>, column: &str) -> Result<f64> {
    let value = value.ok_or_else(|| WaveformError::MissingColumn {
        column: column.to_string(),
    })?;
    value
        .parse::<f64>()
        .map_err(|_| WaveformError::InvalidNumber {
            column: column.to_string(),
            value: value.to_string(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_waveform_csv() {
        let input = "time,input_v\n0.0,0.0\n0.1,5.0\n";
        let parser = SimpleCsvParser;
        let options = CsvParseOptions::new("time", vec!["input_v".to_string()]);

        let waveform = parser.parse_str(input, &options).expect("csv should parse");

        assert_eq!(waveform.time, vec![0.0, 0.1]);
        assert_eq!(waveform.channels[0].samples, vec![0.0, 5.0]);
    }
}
