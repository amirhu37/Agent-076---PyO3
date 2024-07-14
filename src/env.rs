use pyo3::{prelude::*, types::PyDict};
// use pyo3::wrap_pyfunction;
// use derive_more::Display;
// use std::collections::HashMap;
// use pyo3::types::PyString;
// //
// use Python::with_gil as gil;

#[allow(dead_code)]
#[derive(Debug)]
#[pyclass(name = "Env", set_all)]
pub struct Env {
    id: usize,
    name: String,
    action_space: Vec<usize>,
    observation_space: Vec<f64>,
    state: Option<f64>,
}

#[pymethods]
impl Env {
    #[new]
    #[pyo3(signature = (id, name, action_space, observation_space, state))]
    pub fn new(
        id: Option<usize>,
        name: String,
        action_space: Vec<usize>,
        observation_space: Vec<f64>,
        state: Option<f64>,
    ) -> PyResult<Self> {
        Ok(Env {
            id: id.unwrap_or_default(),
            name,
            action_space,
            observation_space,
            state,
        })
    }

    pub fn reset(&mut self) {}

    pub fn step<'py>(
        &mut self,
        py: Python<'py>,
        _action: u8,
    ) -> PyResult<(f64, f64, bool, &'py PyDict)> {
        // Implement your step logic here
        let info = PyDict::new(py);
        // You can populate the info dictionary with key-value pairs if needed
        Ok((0.0, 0.0, false, info))
    }

    pub fn close(&mut self) {}

    fn __str__(&self) -> String {
        format!("Env (name: {})", self.name)
    }
    fn __repr__(&self) {
        self.__str__();
    }
}

// #[pymodule]
// #[pyo3(name = "py_env")]
// fn py_env(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<Env>()?;
//     Ok(())
// }
