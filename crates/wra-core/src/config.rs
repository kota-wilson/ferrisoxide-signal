use serde::Deserialize;

use crate::criteria::Criterion;
use crate::csv::CsvParseOptions;
use crate::error::{Result, WaveformError};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AnalysisConfig {
    pub input: InputConfig,
    #[serde(default)]
    pub filters: Vec<FilterConfig>,
    #[serde(default)]
    pub criteria: Vec<CriterionConfig>,
}

impl AnalysisConfig {
    pub fn csv_options(&self) -> CsvParseOptions {
        CsvParseOptions::new(self.input.time_column.clone(), self.input.channels.clone())
    }

    pub fn criteria(&self) -> Result<Vec<Criterion>> {
        self.criteria
            .iter()
            .map(CriterionConfig::to_criterion)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InputConfig {
    pub time_column: String,
    pub channels: Vec<String>,
    #[serde(default = "default_time_unit")]
    pub time_unit: String,
    #[serde(default = "default_signal_unit")]
    pub signal_unit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FilterConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub window_samples: Option<usize>,
    pub cutoff_hz: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CriterionConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub channel: String,
    pub threshold_v: f64,
}

impl CriterionConfig {
    fn to_criterion(&self) -> Result<Criterion> {
        match self.kind.as_str() {
            "minimum_voltage" => Ok(Criterion::minimum_voltage(
                self.id.clone(),
                self.channel.clone(),
                self.threshold_v,
            )),
            "maximum_voltage" => Ok(Criterion::maximum_voltage(
                self.id.clone(),
                self.channel.clone(),
                self.threshold_v,
            )),
            _ => Err(WaveformError::InvalidParameter {
                name: "criteria.type".to_string(),
                reason: format!("unsupported criterion type `{}`", self.kind),
            }),
        }
    }
}

fn default_time_unit() -> String {
    "s".to_string()
}

fn default_signal_unit() -> String {
    "V".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_config_to_csv_options_and_criteria() {
        let config = AnalysisConfig {
            input: InputConfig {
                time_column: "time".to_string(),
                channels: vec!["input_v".to_string()],
                time_unit: "s".to_string(),
                signal_unit: "V".to_string(),
            },
            filters: vec![FilterConfig {
                kind: "moving_average".to_string(),
                window_samples: Some(2),
                cutoff_hz: None,
            }],
            criteria: vec![CriterionConfig {
                id: "max".to_string(),
                kind: "maximum_voltage".to_string(),
                channel: "input_v".to_string(),
                threshold_v: 5.5,
            }],
        };

        let options = config.csv_options();
        let criteria = config.criteria().expect("criteria should convert");

        assert_eq!(options.time_column, "time");
        assert_eq!(options.channel_columns, vec!["input_v"]);
        assert_eq!(criteria[0].id, "max");
    }
}
