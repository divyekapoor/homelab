use eyre::Result;
use log::info;
use log4rs::config::RawConfig;
use pyo3::prelude::*;
// use std::ops::Drop;
// use derivative::Derivative;
use std::collections::HashMap;
use std::sync::Mutex;
// use std::default::Default;
// use once_cell::sync::OnceCell;
// use dashmap::DashMap;
use pyo3::exceptions::PyRuntimeError;
use zmq2::SocketType;

#[pyclass]
#[derive(Debug, Eq, PartialEq, Hash)]
struct NettleHandle {
    #[pyo3(get)]
    port: i32,
}

struct NettleOperator {
    zmq_socket: zmq2::Socket,
}

struct NettleGlobals {
    zmq_context: zmq2::Context,

    nettle_map: HashMap<NettleHandle, NettleOperator>,
}

impl Default for NettleGlobals {
    fn default() -> Self {
        NettleGlobals {
            zmq_context: zmq2::Context::new(),
            nettle_map: HashMap::new(),
        }
    }
}

#[pymethods]
impl NettleHandle {
    #[new]
    fn new(port: i32) -> Result<Self> {
        let handle = NettleHandle { port: port };
        let nettle_globals = globals::get::<NettleGlobals>();
        nettle_globals.nettle_map.insert(
            handle.clone(),
            NettleOperator {
                zmq_socket: nettle_globals
                    .zmq_context
                    .socket(SocketType::REQ)
                    .map_err(|_| PyRuntimeError::new_err("Unable to create socket"))?,
            },
        );
        Ok(handle)
    }

    fn log(&self) {
        info!("Object is: {:?}", self);
    }

    fn listen(&self) -> Result<Self> {
        let nettle_globals = globals::get::<NettleGlobals>();
        let nettle_operator = nettle_globals
            .nettle_map
            .get(self)
            .ok_or_else(|| PyRuntimeError::new_err(""))?
            .value_mut();
        Ok(*self)
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> Result<String> {
    info!("Sum as string: ({a}, {b})");
    Ok((a + b).to_string())
}

fn init_logging() -> Result<()> {
    let config_str = include_str!("../data/log4rs.yaml");
    let config: RawConfig = serde_yaml::from_str(config_str)?;
    log4rs::init_raw_config(config)?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn nettle_py(_py: Python, m: &PyModule) -> PyResult<()> {
    color_eyre::install()?;
    init_logging()?;
    m.add_class::<NettleHandle>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
