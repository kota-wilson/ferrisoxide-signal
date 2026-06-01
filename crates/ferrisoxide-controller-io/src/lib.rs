//! Host-checkable controller I/O abstraction.
//!
//! This crate models controller inputs and outputs without depending on HALs,
//! RTOS SDKs, hardware bindings, unsafe FFI, Zephyr production support, or
//! hardware timing guarantees.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControllerInputPort {
    pub id: String,
    pub signal: ControllerSignalKind,
    pub unit: String,
}

impl ControllerInputPort {
    pub fn new(
        id: impl Into<String>,
        signal: ControllerSignalKind,
        unit: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            signal,
            unit: unit.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControllerOutputPort {
    pub id: String,
    pub signal: ControllerSignalKind,
    pub unit: String,
    pub safe_value: ControllerIoValue,
}

impl ControllerOutputPort {
    pub fn new(
        id: impl Into<String>,
        signal: ControllerSignalKind,
        unit: impl Into<String>,
        safe_value: ControllerIoValue,
    ) -> Self {
        Self {
            id: id.into(),
            signal,
            unit: unit.into(),
            safe_value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControllerSignalKind {
    AnalogVoltage,
    Digital,
    Pwm,
    Virtual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControllerIoValue {
    Analog { value: f64 },
    Digital { high: bool },
    PwmDuty { duty_cycle: f64 },
    Named { state: String },
}

pub trait ControllerInputProvider {
    fn read_input(&self, port_id: &str) -> Result<ControllerIoValue, ControllerIoError>;
}

pub trait ControllerOutputSink {
    fn write_output(
        &mut self,
        port_id: &str,
        value: ControllerIoValue,
    ) -> Result<(), ControllerIoError>;
    fn output_snapshot(&self) -> BTreeMap<String, ControllerIoValue>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FakeControllerIo {
    inputs: Vec<ControllerInputPort>,
    outputs: Vec<ControllerOutputPort>,
    input_values: BTreeMap<String, ControllerIoValue>,
    output_values: BTreeMap<String, ControllerIoValue>,
}

impl FakeControllerIo {
    pub fn new(
        inputs: Vec<ControllerInputPort>,
        outputs: Vec<ControllerOutputPort>,
    ) -> Result<Self, ControllerIoError> {
        validate_input_ports(&inputs)?;
        validate_output_ports(&outputs)?;
        let output_values = outputs
            .iter()
            .map(|output| (output.id.clone(), output.safe_value.clone()))
            .collect();
        Ok(Self {
            inputs,
            outputs,
            input_values: BTreeMap::new(),
            output_values,
        })
    }

    pub fn set_input(
        &mut self,
        port_id: impl Into<String>,
        value: ControllerIoValue,
    ) -> Result<(), ControllerIoError> {
        let port_id = port_id.into();
        let port = self
            .inputs
            .iter()
            .find(|input| input.id == port_id)
            .ok_or_else(|| ControllerIoError::UnknownInputPort(port_id.clone()))?;
        validate_value_kind(&port.id, port.signal, &value)?;
        self.input_values.insert(port_id, value);
        Ok(())
    }

    pub fn apply_safe_outputs(&mut self) {
        self.output_values = self
            .outputs
            .iter()
            .map(|output| (output.id.clone(), output.safe_value.clone()))
            .collect();
    }
}

impl ControllerInputProvider for FakeControllerIo {
    fn read_input(&self, port_id: &str) -> Result<ControllerIoValue, ControllerIoError> {
        if !self.inputs.iter().any(|input| input.id == port_id) {
            return Err(ControllerIoError::UnknownInputPort(port_id.to_string()));
        }
        self.input_values
            .get(port_id)
            .cloned()
            .ok_or_else(|| ControllerIoError::InputNotSet(port_id.to_string()))
    }
}

impl ControllerOutputSink for FakeControllerIo {
    fn write_output(
        &mut self,
        port_id: &str,
        value: ControllerIoValue,
    ) -> Result<(), ControllerIoError> {
        let port = self
            .outputs
            .iter()
            .find(|output| output.id == port_id)
            .ok_or_else(|| ControllerIoError::UnknownOutputPort(port_id.to_string()))?;
        validate_value_kind(&port.id, port.signal, &value)?;
        self.output_values.insert(port_id.to_string(), value);
        Ok(())
    }

    fn output_snapshot(&self) -> BTreeMap<String, ControllerIoValue> {
        self.output_values.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControllerIoError {
    EmptyInputPortId {
        index: usize,
    },
    EmptyOutputPortId {
        index: usize,
    },
    EmptyUnit {
        port: String,
    },
    DuplicateInputPort {
        port: String,
    },
    DuplicateOutputPort {
        port: String,
    },
    UnknownInputPort(String),
    UnknownOutputPort(String),
    InputNotSet(String),
    ValueKindMismatch {
        port: String,
        expected: ControllerSignalKind,
    },
    InvalidValue {
        port: String,
    },
}

impl fmt::Display for ControllerIoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInputPortId { index } => {
                write!(
                    formatter,
                    "controller input port at index {index} has an empty id"
                )
            }
            Self::EmptyOutputPortId { index } => {
                write!(
                    formatter,
                    "controller output port at index {index} has an empty id"
                )
            }
            Self::EmptyUnit { port } => {
                write!(formatter, "controller port `{port}` has an empty unit")
            }
            Self::DuplicateInputPort { port } => {
                write!(formatter, "duplicate controller input port `{port}`")
            }
            Self::DuplicateOutputPort { port } => {
                write!(formatter, "duplicate controller output port `{port}`")
            }
            Self::UnknownInputPort(port) => {
                write!(formatter, "unknown controller input port `{port}`")
            }
            Self::UnknownOutputPort(port) => {
                write!(formatter, "unknown controller output port `{port}`")
            }
            Self::InputNotSet(port) => write!(formatter, "controller input `{port}` has no value"),
            Self::ValueKindMismatch { port, expected } => write!(
                formatter,
                "controller port `{port}` value does not match expected {expected:?} signal"
            ),
            Self::InvalidValue { port } => {
                write!(formatter, "controller port `{port}` value is invalid")
            }
        }
    }
}

impl std::error::Error for ControllerIoError {}

fn validate_input_ports(inputs: &[ControllerInputPort]) -> Result<(), ControllerIoError> {
    let mut ids = BTreeSet::new();
    for (index, input) in inputs.iter().enumerate() {
        if input.id.is_empty() {
            return Err(ControllerIoError::EmptyInputPortId { index });
        }
        if input.unit.is_empty() {
            return Err(ControllerIoError::EmptyUnit {
                port: input.id.clone(),
            });
        }
        if !ids.insert(input.id.clone()) {
            return Err(ControllerIoError::DuplicateInputPort {
                port: input.id.clone(),
            });
        }
    }
    Ok(())
}

fn validate_output_ports(outputs: &[ControllerOutputPort]) -> Result<(), ControllerIoError> {
    let mut ids = BTreeSet::new();
    for (index, output) in outputs.iter().enumerate() {
        if output.id.is_empty() {
            return Err(ControllerIoError::EmptyOutputPortId { index });
        }
        if output.unit.is_empty() {
            return Err(ControllerIoError::EmptyUnit {
                port: output.id.clone(),
            });
        }
        if !ids.insert(output.id.clone()) {
            return Err(ControllerIoError::DuplicateOutputPort {
                port: output.id.clone(),
            });
        }
        validate_value_kind(&output.id, output.signal, &output.safe_value)?;
    }
    Ok(())
}

fn validate_value_kind(
    port: &str,
    signal: ControllerSignalKind,
    value: &ControllerIoValue,
) -> Result<(), ControllerIoError> {
    match (signal, value) {
        (ControllerSignalKind::AnalogVoltage, ControllerIoValue::Analog { value }) => {
            if value.is_finite() {
                Ok(())
            } else {
                Err(ControllerIoError::InvalidValue {
                    port: port.to_string(),
                })
            }
        }
        (ControllerSignalKind::Digital, ControllerIoValue::Digital { .. }) => Ok(()),
        (ControllerSignalKind::Pwm, ControllerIoValue::PwmDuty { duty_cycle }) => {
            if duty_cycle.is_finite() && (0.0..=1.0).contains(duty_cycle) {
                Ok(())
            } else {
                Err(ControllerIoError::InvalidValue {
                    port: port.to_string(),
                })
            }
        }
        (ControllerSignalKind::Virtual, ControllerIoValue::Named { .. }) => Ok(()),
        _ => Err(ControllerIoError::ValueKindMismatch {
            port: port.to_string(),
            expected: signal,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_controller_io_reads_inputs_and_writes_outputs() {
        let mut io = fake_io();
        io.set_input("command", ControllerIoValue::Analog { value: 5.0 })
            .expect("set command");

        assert_eq!(
            io.read_input("command").expect("read command"),
            ControllerIoValue::Analog { value: 5.0 }
        );

        io.write_output("heater_enable", ControllerIoValue::Digital { high: true })
            .expect("write heater");
        io.write_output(
            "actuator_pwm",
            ControllerIoValue::PwmDuty { duty_cycle: 0.75 },
        )
        .expect("write pwm");

        let outputs = io.output_snapshot();
        assert_eq!(
            outputs.get("heater_enable"),
            Some(&ControllerIoValue::Digital { high: true })
        );
        assert_eq!(
            outputs.get("actuator_pwm"),
            Some(&ControllerIoValue::PwmDuty { duty_cycle: 0.75 })
        );
    }

    #[test]
    fn fake_controller_io_starts_and_resets_to_safe_outputs() {
        let mut io = fake_io();
        let outputs = io.output_snapshot();
        assert_eq!(
            outputs.get("heater_enable"),
            Some(&ControllerIoValue::Digital { high: false })
        );

        io.write_output("heater_enable", ControllerIoValue::Digital { high: true })
            .expect("write heater");
        io.apply_safe_outputs();

        assert_eq!(
            io.output_snapshot().get("heater_enable"),
            Some(&ControllerIoValue::Digital { high: false })
        );
    }

    #[test]
    fn fake_controller_io_rejects_unknown_or_invalid_values() {
        let mut io = fake_io();

        let error = io.read_input("missing").expect_err("unknown input");
        assert!(matches!(error, ControllerIoError::UnknownInputPort(_)));

        let error = io
            .set_input("command", ControllerIoValue::Digital { high: true })
            .expect_err("wrong input kind");
        assert!(matches!(error, ControllerIoError::ValueKindMismatch { .. }));

        let error = io
            .write_output(
                "actuator_pwm",
                ControllerIoValue::PwmDuty { duty_cycle: 1.25 },
            )
            .expect_err("bad pwm");
        assert!(matches!(error, ControllerIoError::InvalidValue { .. }));
    }

    #[test]
    fn fake_controller_io_rejects_duplicate_ports() {
        let inputs = vec![
            ControllerInputPort::new("command", ControllerSignalKind::AnalogVoltage, "V"),
            ControllerInputPort::new("command", ControllerSignalKind::Digital, "bool"),
        ];
        let error = FakeControllerIo::new(inputs, outputs()).expect_err("duplicate input");
        assert!(matches!(
            error,
            ControllerIoError::DuplicateInputPort { .. }
        ));
    }

    fn fake_io() -> FakeControllerIo {
        FakeControllerIo::new(inputs(), outputs()).expect("fake controller I/O")
    }

    fn inputs() -> Vec<ControllerInputPort> {
        vec![
            ControllerInputPort::new("command", ControllerSignalKind::AnalogVoltage, "V"),
            ControllerInputPort::new("feedback", ControllerSignalKind::Digital, "bool"),
        ]
    }

    fn outputs() -> Vec<ControllerOutputPort> {
        vec![
            ControllerOutputPort::new(
                "heater_enable",
                ControllerSignalKind::Digital,
                "bool",
                ControllerIoValue::Digital { high: false },
            ),
            ControllerOutputPort::new(
                "actuator_pwm",
                ControllerSignalKind::Pwm,
                "percent",
                ControllerIoValue::PwmDuty { duty_cycle: 0.0 },
            ),
        ]
    }
}
