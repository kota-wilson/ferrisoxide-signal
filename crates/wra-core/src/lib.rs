//! Core library for Waveform Reconstructor and Analyzer.

pub mod analysis;
pub mod config;
pub mod criteria;
pub mod csv;
pub mod error;
pub mod filter;
pub mod model;
pub mod report;

pub use error::{Result, WaveformError};
