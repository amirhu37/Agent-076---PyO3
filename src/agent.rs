
// use ndarray::Array0;
use pyo3::prelude::*;
// use pyo3::types::PyList;
use std::sync::atomic::{AtomicU16, Ordering};
// Atomic counter for generating unique IDs
static COUNTER: AtomicU16 = AtomicU16::new(1);
// use pyo3::types::PyAny;
// use pyo3::wrap_pyfunction;
// use numpy::PyArray1;
// use numpy::ToPyArray;
// use numpy::IntoPyArray;





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
// #[derive(Debug, Clone)]
#[pyclass(name = "Agent", unsendable, subclass, sequence)]
#[pyo3(text_signature = "(name, actions ,  utility = 0.0)" , dict) ]
pub struct Agent {
    #[pyo3(get, set)]
    id: u16,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    utility: f64,
    #[pyo3(get, set)]
    actions: PyObject,
}

#[pymethods]
impl Agent {
    #[new]
    pub fn new<'py>(name: String, actions: PyObject, utility: Option<f64>) -> PyResult<Self> {
        let current: u16 = COUNTER.load(Ordering::SeqCst).into();
        COUNTER.fetch_add(1, Ordering::SeqCst);

        // let actions_vec  = match actions {        };
            // return Err(pyo3::exceptions::PyValueError::new_err("Expected a numpy array or list"));
        // };

        let obj = Self {
            id: current,
            name,
            actions,
            utility: utility.unwrap_or(0.0),
        };

        Ok(obj)
    }




    fn rule(&mut self, _observation: f64)-> PyObject {
        // Python::with_gil(|py| {None.to_object(py)} )
        todo!()
    }

    fn returns(&self)->PyObject{todo!()}

    fn __str__(&self) -> String {
        format!(
            "Agent(Id: {}, Name: {}, Actions: {:?}, Utility: {:?})",
            self.id, self.name, self.actions,  self.utility
        )
    }

    fn __repr__(&self) {
        self.__str__();
    }

}

//  : Python::with_gil(|py|{ actions.to_object(py).extract(py).unwrap() }),
