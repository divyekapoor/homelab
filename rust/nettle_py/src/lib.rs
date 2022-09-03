use log::info;
use log4rs::config::RawConfig;
use pyo3::prelude::*;
use eyre::{Result};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> Result<String> {
    info!("Sum as string: ({a}, {b})");
    Ok((a + b).to_string())
}

#[pyfunction]
fn init_logging() -> Result<()> {
    let config_str = include_str!("../data/log4rs.yaml");
    let config: RawConfig = serde_yaml::from_str(config_str)?;
    log4rs::init_raw_config(config)?;
    Ok(())
}

#[pyfunction]
fn init_logging_with(config_str: &str) -> Result<()> {
    let config : RawConfig = serde_yaml::from_str(config_str)?;
    log4rs::init_raw_config(config)?;

    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn nettle_py(_py: Python, m: &PyModule) -> PyResult<()> {
    color_eyre::install()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(init_logging, m)?)?;
    m.add_function(wrap_pyfunction!(init_logging_with, m)?)?;
    Ok(())
}
