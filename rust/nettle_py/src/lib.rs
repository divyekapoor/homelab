use eyre::Result;
use log::info;
use log4rs::config::RawConfig;
use pyo3::prelude::*;
// use std::ops::Drop;
// use derivative::Derivative;
// use std::collections::HashMap;
// use std::sync::Mutex;
// use std::default::Default;
// use once_cell::sync::OnceCell;
// use dashmap::DashMap;
// use pyo3::exceptions::PyRuntimeError;
use zmq::SocketType;
use derivative::Derivative;

#[pyclass]
#[derive(Derivative)]
#[derivative(Debug, Eq, PartialEq, Hash)]
struct NettleHandle {
    #[pyo3(get)]
    port: i32,

    #[derivative(PartialEq="ignore")]
    #[derivative(Hash="ignore")]
    #[derivative(Debug="ignore")]
    zmq_socket: zmq::Socket
}

#[pymethods]
impl NettleHandle {
    #[new]
    fn new(port: i32) -> Result<Self> {
        let zmq_context = zmq::Context::new();
        let zmq_socket = zmq_context.socket(SocketType::REQ)?;

        let handle = NettleHandle { port: port, zmq_socket: zmq_socket };
        Ok(handle)
    }

    fn log(&self) {
        info!("Object is: {:?}", self);
    }

    fn listen(&self) -> Result<()> {
        self.zmq_socket.bind(
            &("tcp://0.0.0.0:".to_owned() + &self.port.to_string()))?;
        info!("Listening to port: {:?}", self.port);
        Ok(())
    }
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
    Ok(())
}
