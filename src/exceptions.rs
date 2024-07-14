
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::fmt;

#[pyclass(extends=PyException)]
#[derive(Debug)]
pub struct IDValueError {
    msg: String,
}

#[pymethods]
impl IDValueError {
    #[new]
    fn new(msg: String) -> Self {
        IDValueError { msg }
    }

    fn __str__(&self) -> String {
        format!("IDValueError: {}", self.msg)
    }
}

impl std::error::Error for IDValueError {}

impl fmt::Display for IDValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IDValueError: {}", self.msg)
    }
}

impl std::convert::From<IDValueError> for PyErr {
    fn from(err: IDValueError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

impl IDValueError {
    pub fn new_err(err: String) -> PyErr {
        IDValueError { msg: err }.into()
    }
}
