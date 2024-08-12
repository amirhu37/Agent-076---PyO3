use exceptions::IDValueError;
use numpy::PyArray2;
use pyo3::{prelude::*, types::IntoPyDict};
use pyo3::Bound as PyBound;

// Declare the modules
pub mod agent;
pub mod env;
pub mod exceptions;

use agent::Agent;
use env::Env;

pub type Np2darray = Py<PyArray2<usize>>;

pub fn np_zeros(r: u8, c: u8, is_fortran: bool) -> Np2darray {
    let dims = [r as usize, c as usize]; // Convert u8 to usize for dimensions
    Python::with_gil(|py| {
        let pyarray = PyArray2::zeros_bound(py, dims, is_fortran).unbind();
        pyarray.to_owned()
    })
}

pub fn datatype<T>(_: &T) -> &str {
    let rtn: &str = std::any::type_name::<T>();
    // let mut e: Vec<&str> = rtn.split("::").collect::<Vec<&str>>();
    // e.reverse();
    // e[0]
    rtn
}
pub fn run_py(value: &PyObject, command: &str) -> String {
    let var = Python::with_gil(|py| {
        let locals = [("value", value)].into_py_dict_bound(py);
        let c = py.eval_bound(command, None, Some(&locals)).expect("running Command Field").to_string();
        c
    });
    var
}





#[macro_export]
macro_rules! add_class {
    ($module : ident , $($class : ty), +) => {
        $(
            $module.add_class::<$class>()?;
        )+

    };
}
#[macro_export]

macro_rules! add_function {
    ($module : ident , $($function : ident), +) => {
        $(
           $module.add_wrapped(wrap_pyfunction!($function))?;
        )+
    };
}



#[pymodule]
#[pyo3(name = "society")]
pub fn society(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    add_class!(m, Agent, Env, IDValueError);
    // m.add_class::<Agent>()?; 
    // m.add_class::<Env>()?;
    // m.add_class::<IDValueError>()?;
    Ok(())
}
