#[derive(Debug, Clone, PartialEq)]
pub struct Criterion {
    pub id: String,
    pub check: CriterionCheck,
}

impl Criterion {
    pub fn minimum_voltage(
        id: impl Into<String>,
        channel: impl Into<String>,
        threshold_v: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::MinimumVoltage {
                channel: channel.into(),
                threshold_v,
            },
        }
    }

    pub fn maximum_voltage(
        id: impl Into<String>,
        channel: impl Into<String>,
        threshold_v: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::MaximumVoltage {
                channel: channel.into(),
                threshold_v,
            },
        }
    }

    pub fn channel(&self) -> &str {
        self.check.channel()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CriterionCheck {
    MinimumVoltage { channel: String, threshold_v: f64 },
    MaximumVoltage { channel: String, threshold_v: f64 },
}

impl CriterionCheck {
    pub fn channel(&self) -> &str {
        match self {
            Self::MinimumVoltage { channel, .. } | Self::MaximumVoltage { channel, .. } => channel,
        }
    }
}
