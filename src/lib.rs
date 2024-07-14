use exceptions::IDValueError;
use numpy::PyArray2;
use pyo3::prelude::*;

// Declare the modules (assuming you have them in separate files)
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




#[pymodule]
#[pyo3(name = "society")]
pub fn society(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Agent>()?; // Add the Agent class
    m.add_class::<Env>()?;
    m.add_class::<IDValueError>()?;
    Ok(())
}
