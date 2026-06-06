# ferrisoxide-workflow Architecture

Date: 2026-06-06

## Responsibility

`ferrisoxide-workflow` owns reusable desktop workflow orchestration for CLI and GUI callers. It converts command-style requests into source inspection, config scaffolding, analysis, plotting, rule-package export, simulation, batch execution, transform catalog output, workflow templates, and evaluation bundles.

## Non-Goals

- Native UI widgets, terminal entrypoint behavior, core transform implementation, schema ownership, live DAQ vendor SDKs, HAL/RTOS bindings, package signing, release publication, or certification evidence.

## Public Boundary

| Area | Public API |
|---|---|
| Command dispatcher | `run(args: Vec<String>) -> Result<String, String>` |
| Source workflow | `InspectSourceRequest`, `SourceInspection`, `inspect_source`, `load_csv_headers`, `render_source_inspection_output` |
| Config workflow | `ScaffoldConfigRequest`, `scaffold_csv_config`, workflow template APIs |
| Analysis workflow | `AnalyzeCsvRequest`, `analyze_csv` |
| Bundle workflow | `EvaluateBundleRequest`, `WorkflowBundleOutput`, `evaluate_bundle` |
| Plot workflow | `CsvPlotSeriesRequest`, `WorkflowPlotSeries`, `WorkflowPlotPoint`, `load_csv_plot_series` |
| Source modes | `WorkflowSourceMode::Csv`, `WorkflowSourceMode::Simulation` |

## Flowchart

```mermaid
flowchart TD
    A["CLI args or GUI request structs"] --> B["run() dispatcher or typed public functions"]
    B --> C{"Workflow path"}
    C --> D["inspect-source<br/>CSV headers/waveform or simulation fixture"]
    C --> E["scaffold-config<br/>starter AnalysisConfig TOML"]
    C --> F["analyze / plot<br/>CSV + config"]
    C --> G["export-rule-package<br/>RulePackage and manifest"]
    C --> H["simulate / evaluate-bundle<br/>control + verification + channel map"]
    C --> I["batch / transforms / templates"]
    F --> J["ferrisoxide-core<br/>parse, transform, feature, criteria, report"]
    F --> K["ferrisoxide-plot<br/>optional SVG"]
    G --> L["ferrisoxide-rule-schema<br/>rules and checksum artifacts"]
    H --> M["ferrisoxide-daq<br/>fixture frames"]
    H --> N["ferrisoxide-simulator<br/>controller trace"]
    H --> O["ferrisoxide-control-schema and ferrisoxide-verification-schema"]
    D --> P["SourceInspection text/json"]
    E --> Q["Generated TOML text"]
    F --> R["Report text/json or plot series"]
    G --> S["Rule package files"]
    H --> T["WorkflowBundleOutput and artifact paths"]
    B -.-> U["Caller-visible error"]
```

## Important Error Paths

- Missing required CLI flags or request fields return `String` errors before file work begins.
- File reads, TOML/JSON parsing, config validation, waveform parsing, transform failures, report rendering, bundle directory conflicts, and artifact write failures are surfaced as caller-visible strings.
- Simulation workflows require control config, verification config, channel map, mode, and source data; missing or invalid inputs stop before bundle completion.
- `write_output_file` and related helpers protect against accidental overwrite unless overwrite is requested.

## Validation

- `cargo test -p ferrisoxide-workflow`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
