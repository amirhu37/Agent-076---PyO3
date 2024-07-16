// use ndarray::Array0;
use pyo3::{exceptions::PyValueError, prelude::*};
// use pyo3::types::PyList;
use std::sync::atomic::{AtomicU16, Ordering};

use crate::env::_py_run_as_string;

// Atomic counter for generating unique IDs
static COUNTER: AtomicU16 = AtomicU16::new(1);
/// Creates a new agent.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the agent.
/// * `actions` - A vector of unsigned 16-bit integers representing actions.
/// * `utility` - A 64-bit floating point number representing the utility of the agent.
/// # Returns
///     Object of Agent
/// *          `id` - An optional 16-bit unsigned integer representing the ID of the agent.
/// Represents an agent with specific attributes.
#[pyclass(name = "Agent", unsendable, subclass, sequence, dict)]
#[pyo3(text_signature = "(name : str, actions : list[int] | np.ndarray ,  utility : list[int] | np.ndarray)")]
pub struct Agent {
    #[pyo3(get, set, name = "id")]
    id: u16,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    utility: PyObject,
    #[pyo3(get, set)]
    actions: PyObject,
}

#[pymethods]
impl Agent {
    #[new]
    pub fn new<'py>(
            name: String, 
            actions: PyObject, 
            utility: PyObject
        ) -> PyResult<Self> {
        let current: u16 = COUNTER.load(Ordering::SeqCst).into();
        COUNTER.fetch_add(1, Ordering::SeqCst);

        let action_types: String = _py_run_as_string(&actions, "type(value)");
        let utility_types: String = _py_run_as_string(&actions, "type(value)");

        //  let __dict__ = locals;
        let ac: Py<PyAny> = match action_types.as_str() {
            "<class 'list'>" => actions,
            "<class 'tuple'>" => actions,
            "<class 'numpy.ndarray'>" => actions,
            "<class 'set'>" => actions,
            _ => {
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Invalid type of actions {}",
                    action_types
                )))
            }
        };
        let ut: Py<PyAny> = match utility_types.as_str() {
            "<class 'list'>" => utility,
            "<class 'numpy.ndarray'>" => utility,
            _ => {
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Invalid type of actions {}",
                    utility_types
                )))
            }};

        Ok(Self {
            id: current,
            name,
            actions: ac,
            utility: ut,
        })
    }
    #[pyo3(text_signature = "($cls, _action : int | np.u16 )")]
    fn policy(&mut self, _action: PyObject) -> Option<PyObject> {
        None
    }
    #[pyo3(text_signature = "($cls, _dicount_factor : float, _reward : float)")]
    fn returns(&self, _dicount_factor: f32, _reward: f32) -> Option<PyObject> {
        None
    }
    fn __str__(&self) -> String {
        let utility_shape: String = _py_run_as_string(&self.utility, "np.shape(value)");
        let actions_shape: String = _py_run_as_string(&self.actions, "np.shape(value)");


        format!(
            "Agent(Id: {}, Name: {}, Actions-area: {}, Utility-table: {})",
            self.id, self.name, actions_shape, utility_shape
        )
    }

    fn __repr__(&self) {
        self.__str__();
    }
}

//  : Python::with_gil(|py|{ actions.to_object(py).extract(py).unwrap() }),
