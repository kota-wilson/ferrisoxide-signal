use crate::analysis::{AnalysisResult, Outcome};
use crate::error::{Result, WaveformError};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisReport {
    pub input_name: String,
    pub results: Vec<AnalysisResult>,
}

impl AnalysisReport {
    pub fn overall_outcome(&self) -> Outcome {
        if self
            .results
            .iter()
            .any(|result| result.outcome == Outcome::Fail)
        {
            Outcome::Fail
        } else {
            Outcome::Pass
        }
    }

    pub fn render_text(&self) -> String {
        let mut output = String::new();
        output.push_str("Waveform Analysis Report\n");
        output.push_str(&format!("Input: {}\n", self.input_name));
        output.push_str(&format!("Overall: {:?}\n", self.overall_outcome()));
        output.push_str("Criteria:\n");

        for result in &self.results {
            output.push_str(&format!(
                "- {}: {:?} measured={:.6} {} threshold={:.6} {}\n",
                result.criterion_id,
                result.outcome,
                result.measured_value,
                result.unit,
                result.threshold,
                result.unit
            ));
        }

        output
    }

    pub fn render_json_pretty(&self) -> Result<String> {
        let document = JsonReport {
            input_name: &self.input_name,
            overall_outcome: self.overall_outcome(),
            results: &self.results,
        };
        serde_json::to_string_pretty(&document).map_err(|error| {
            WaveformError::ReportSerialization {
                message: error.to_string(),
            }
        })
    }
}

#[derive(Serialize)]
struct JsonReport<'a> {
    input_name: &'a str,
    overall_outcome: Outcome,
    results: &'a [AnalysisResult],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_text_report() {
        let report = AnalysisReport {
            input_name: "fixture.csv".to_string(),
            results: vec![AnalysisResult {
                criterion_id: "max".to_string(),
                outcome: Outcome::Pass,
                measured_value: 5.0,
                threshold: 5.5,
                unit: "V".to_string(),
                reason: "ok".to_string(),
            }],
        };

        let rendered = report.render_text();

        assert!(rendered.contains("Waveform Analysis Report"));
        assert!(rendered.contains("Overall: Pass"));
        assert!(rendered.contains("max"));
    }

    #[test]
    fn renders_json_report() {
        let report = AnalysisReport {
            input_name: "fixture.csv".to_string(),
            results: vec![AnalysisResult {
                criterion_id: "max".to_string(),
                outcome: Outcome::Pass,
                measured_value: 5.0,
                threshold: 5.5,
                unit: "V".to_string(),
                reason: "ok".to_string(),
            }],
        };

        let rendered = report.render_json_pretty().expect("json should render");

        assert!(rendered.contains("\"overall_outcome\": \"pass\""));
        assert!(rendered.contains("\"criterion_id\": \"max\""));
    }
}
