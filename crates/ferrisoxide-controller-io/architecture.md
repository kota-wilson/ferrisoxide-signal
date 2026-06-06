# ferrisoxide-controller-io Architecture

Date: 2026-06-06

## Responsibility

`ferrisoxide-controller-io` owns host-checkable controller input/output abstractions. It defines typed input/output ports, signal/value kinds, input provider and output sink traits, and a fake controller I/O implementation for deterministic tests and workflow simulations.

## Non-Goals

- HAL binding, RTOS SDK integration, hardware timing guarantees, unsafe FFI, Zephyr production support, live controller execution, or certification evidence.

## Public Boundary

| Area | Public API |
|---|---|
| Ports | `ControllerInputPort`, `ControllerOutputPort`, `ControllerSignalKind` |
| Values | `ControllerIoValue` |
| Traits | `ControllerInputProvider`, `ControllerOutputSink` |
| Fake I/O | `FakeControllerIo::new`, `set_input`, `apply_safe_outputs`, `read_input`, `write_output`, `output_snapshot` |
| Errors | `ControllerIoError` |

## Flowchart

```mermaid
flowchart TD
    A["Input and output port definitions"] --> B["Validate port IDs, units,<br/>duplicates, and safe values"]
    B --> C["FakeControllerIo"]
    D["set_input(port, value)"] --> E["Validate port exists and value kind"]
    E --> C
    F["read_input(port)"] --> G["ControllerInputProvider"]
    C --> G
    H["write_output(port, value)"] --> I["Validate output exists and value kind"]
    I --> J["ControllerOutputSink"]
    C --> J
    J --> K["output_snapshot"]
    B -.-> L["Error output"]
    E -.-> L
    I -.-> L
```

## Important Error Paths

- Construction rejects empty port IDs, empty units, duplicate input/output ports, and invalid safe output values.
- Reads reject unknown or unset inputs.
- Writes reject unknown outputs, value-kind mismatches, non-finite analog/PWM values, or invalid PWM duty cycles.

## Validation

- `cargo test -p ferrisoxide-controller-io`
- `cargo clippy -p ferrisoxide-controller-io --all-targets -- -D warnings`
