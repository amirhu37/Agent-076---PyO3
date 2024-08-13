// use ndarray::Array0;
use pyo3::{prelude::*, types::PyDict};
// use pyo3::types::PyList;
use std::sync::atomic::{AtomicUsize, Ordering};
use pyo3::Bound as PyBound;

// use crate::run_py;

// Atomic counter for generating unique IDs
static COUNTER: AtomicUsize = AtomicUsize::new(1);
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
#[pyclass(name = "Agent",
 unsendable, 
 subclass,
  sequence,
   dict)]

// #[pyo3(text_signature = "(name : str, actions : list[int] | np.ndarray ,  utility : list[int] | np.ndarray)")]
pub struct Agent {
    #[pyo3(get, set, name = "id")]
    id: usize,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    args : PyObject ,
    #[pyo3(get, set)]
    kwargs : PyObject,
}

#[pymethods]
impl Agent {
    #[new]
    #[pyo3(signature = (name, *args , **kwargs)
        ,text_signature = "(name : str, actions : list[int] | np.ndarray )")]
    pub fn new<'py>(
            _py: Python,
            name: String, 
            // actions_size: PyObject, 
            args: &Bound<'_, PyAny>,
            kwargs: Option<&Bound<'_, PyAny>>,
        ) -> PyResult<Self> {
        let current: usize = COUNTER.load(Ordering::SeqCst).into();
        COUNTER.fetch_add(1, Ordering::SeqCst);

        Ok(Self {
            id: current,
            name: name ,
            args : args.clone().unbind() ,
            kwargs : kwargs.unwrap().clone().unbind()
        })
    }


    #[pyo3(text_signature = "($cls, _action : int | np.u16 )")]    
    fn policy(_slf: &PyBound<Self>, _action: PyObject) -> Option<PyObject> {
        None
    }
    #[pyo3(text_signature = "($cls, _dicount_factor : float, _reward : Any)")]
    fn returns(_slf: &PyBound<Self>, _dicount_factor: f32, _reward: PyBound<PyAny>) -> Option<PyObject> {
        None
    }
    fn __str__(slf: &PyBound<Self> , py : Python<'_>) -> String {
        let class_name = slf .get_type().qualname().unwrap();
        let kw: Py<PyDict> = slf.borrow().kwargs.extract(py).expect("kw");
        // let args: Py<PyTuple> = slf.borrow().args.extract(py).expect("arg");
        let kw = kw.call_method0(py, "keys").expect("faild keys");
        // let extract = kw.downcast_bound(py);
        // let t: Py<PyList> = extract.expect("msg").clone().unbind() ; 
             

        format!(
            "{}(id = {}, Name = {} , {})",
            class_name,
             slf.borrow().id, 
             slf.borrow().name,
             kw,//kw.call_method0(py, "keys").expect("faild keys") , 
        )
    }

    fn __repr__(slf: &PyBound<Self>)-> PyResult<String> {
        let class_name = slf .get_type().qualname()?;
        Ok(class_name)
    }
}
