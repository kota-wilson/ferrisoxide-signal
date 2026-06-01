# Controller I/O Abstraction

Status: implemented host-checkable boundary for M9-005 / issue #81.

Crate: `crates/ferrisoxide-controller-io`

## Purpose

The controller I/O abstraction separates portable controller logic from hardware bindings. It gives desktop tests and future runtime adapters a small common shape for controller inputs, controller outputs, safe output values, and structured I/O errors.

Use this crate for:

- host-checkable fake controller inputs,
- host-checkable fake controller outputs,
- output safe-state reset tests,
- future simulator-to-I/O mapping,
- future HAL or RTOS adapter boundaries.

Do not use this crate for:

- hardware HAL integration,
- RTOS SDK integration,
- unsafe FFI,
- Zephyr production support,
- live GPIO/PWM/ADC access,
- hardware timing guarantees,
- certification evidence.

## Current API Boundary

The crate defines:

| Type | Purpose |
|---|---|
| `ControllerInputPort` | Input port ID, signal kind, and unit. |
| `ControllerOutputPort` | Output port ID, signal kind, unit, and safe value. |
| `ControllerSignalKind` | Analog voltage, digital, PWM, or virtual signal classification. |
| `ControllerIoValue` | Analog, digital, PWM duty, or named-state value. |
| `ControllerInputProvider` | Trait for reading controller input values. |
| `ControllerOutputSink` | Trait for writing controller output values and collecting output snapshots. |
| `FakeControllerIo` | Host-checkable fake implementation for tests and desktop simulation. |
| `ControllerIoError` | Structured errors for invalid ports and values. |

## Fake I/O Example

```rust
use ferrisoxide_controller_io::{
    ControllerInputPort, ControllerOutputPort, ControllerSignalKind, ControllerIoValue,
    ControllerInputProvider, ControllerOutputSink, FakeControllerIo,
};

let inputs = vec![
    ControllerInputPort::new("command", ControllerSignalKind::AnalogVoltage, "V"),
];
let outputs = vec![
    ControllerOutputPort::new(
        "heater_enable",
        ControllerSignalKind::Digital,
        "bool",
        ControllerIoValue::Digital { high: false },
    ),
];

let mut io = FakeControllerIo::new(inputs, outputs)?;
io.set_input("command", ControllerIoValue::Analog { value: 5.0 })?;
let command = io.read_input("command")?;
io.write_output("heater_enable", ControllerIoValue::Digital { high: true })?;
```

## Validation Rules

`FakeControllerIo::new()` validates:

- input and output IDs are non-empty,
- units are non-empty,
- input IDs are unique,
- output IDs are unique,
- output safe values match output signal kinds,
- PWM safe values are finite and between `0.0` and `1.0`.

Runtime operations validate:

- reads reference known input ports,
- writes reference known output ports,
- set input values match input signal kinds,
- output values match output signal kinds,
- analog values are finite,
- PWM values are finite and within `0.0..=1.0`.

## Future Adapter Gates

Hardware adapters are intentionally out of scope. Before any HAL, RTOS SDK, unsafe FFI, Zephyr production support, GPIO/PWM/ADC access, or timing guarantee is added, run these gates:

| Gate | Required Evidence |
|---|---|
| Dependency Gate | HAL/SDK license, version, transitive dependency, and target support review. |
| Environment Gate | Project-local build setup, target triple, no global install by default, and cleanup plan. |
| Security Gate | Unsafe FFI review, device permission review, and no secret-bearing configuration. |
| Timing Gate | Explicit timing assumptions, jitter limits, and no overclaim beyond measured evidence. |
| V&V Gate | Host fake parity tests and target adapter conformance tests over the same logical I/O vectors. |

## Current Limits

This crate does not yet connect simulator actions to controller output sinks, does not map DAQ frames to input ports, and does not own any target hardware adapter. The desktop simulation workflow and parity-test issues should connect these boundaries later.
