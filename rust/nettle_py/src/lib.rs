use log::info;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    info!("Sum as string: ({a}, {b})");
    Ok((a + b).to_string())
}

#[pyfunction]
fn init_logging() -> PyResult<()> {
    let config_str = include_str!("../data/log4rs.yaml");
    let config = serde_yaml::from_str(config_str).expect(
        "expected config_str to be available");
    log4rs::init_raw_config(config).expect(
        "log4rs initiatlization failed!");
}

/// A Python module implemented in Rust.
#[pymodule]
fn nettle_py(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
