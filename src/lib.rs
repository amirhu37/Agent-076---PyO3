use exceptions::IDValueError;
use numpy::PyArray2;
use pyo3::{prelude::*, types::IntoPyDict};


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
        let pyarray = PyArray2::zeros(py, dims, is_fortran);
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
pub fn _py_run_as_string(value: &PyObject, command: &str) -> String {
    let var = Python::with_gil(|py| {
        let locals = [("value", value)].into_py_dict(py);
        let c = py.eval(command, None, Some(locals)).unwrap().to_string();
        c
    });
    var
}


#[pymodule]
#[pyo3(name = "society")]
pub fn society(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Agent>()?; 
    m.add_class::<Env>()?;
    m.add_class::<IDValueError>()?;
    Ok(())
}
