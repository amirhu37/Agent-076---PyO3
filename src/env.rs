use std::sync::atomic::{AtomicU16, Ordering};

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::PyDict,
};

use crate::_py_run_as_string;
static COUNTER: AtomicU16 = AtomicU16::new(1);

#[allow(dead_code)]
#[derive(Debug)]
#[pyclass(name = "Env", set_all, subclass, get_all, dict, sequence, unsendable)]
#[pyo3(
    text_signature = "( name : str, action_space : list[int] | np.ndarray, observation_space : np.ndarra)"
)]
pub struct Env {
    id: u16,
    name: String,
    action_space: PyObject,
    observation_space: PyObject,
}

#[pymethods]
impl Env {
    #[new]
    #[pyo3(signature = ( name, action_space, observation_space))]
    pub fn new<'py>(
        name: String,
        action_space: PyObject,
        observation_space: PyObject,
    ) -> PyResult<Self> {
        let current: u16 = COUNTER.load(Ordering::SeqCst).into();
        COUNTER.fetch_add(1, Ordering::SeqCst);

        let ac_type: String = _py_run_as_string(&action_space, "type(value)");
        let ob_type: String = _py_run_as_string(&observation_space, "type(value)");

        let ac: Py<PyAny> = match ac_type.as_str() {
            "<class 'list'>" => action_space,
            "<class 'numpy.ndarray'>" => action_space,

            _ => {
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Invalid type of action space type {}, Only `list` or `numpy.ndarray`",
                    ac_type
                )))
            }
        };
        let env_space: Py<PyAny> = match ob_type.as_str() {
            "<class 'numpy.ndarray'>" => observation_space,
            _ => {
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Invalid type of env space {} , only `numpy.ndarray` is acceptable",
                    ob_type
                )))
            }
        };
        Ok(Self {
            id: current,
            name,
            action_space: ac,
            observation_space: env_space,
        })
    }

    pub fn reset(&mut self) {}
    ///# step
    /// ## input
    /// #### state
    /// #### action
    /// ## output
    /// ####   reward
    /// ####    done
    #[pyo3(text_signature = "($cls, _state, _action)")]
    pub fn step<'py>(
        &mut self,
        py: Python<'py>,
        _state: PyObject,
        _action: u8,
    ) -> PyResult<(PyObject, bool, &'py PyDict)> {
        let reward = 0.0.to_object(py);
        let info: &PyDict = PyDict::new(py);
        let done: bool = false;

        Ok((reward, done, info))
    }

    pub fn close(&mut self) {}

    fn __str__(&self) -> String {
        // let observation_space = self.observation_space.clone();
        let observation_space_size = _py_run_as_string(&self.observation_space, "np.shape(value)");
        let action_space_size = _py_run_as_string(&self.action_space, "np.shape(value)");

        format!(
            "Env(name: {}, action space size : {}, envirnment shape {})",
            self.name, action_space_size, observation_space_size
        )
    }
    fn __repr__(&self) {
        self.__str__();
    }
}

