use crate::PyBound;
use std::sync::atomic::{AtomicU16, Ordering};
pub type Ndarray<Dimen> = ArrayBase<OwnedRepr<f64>, Dim<Dimen>>;
use ndarray::{ArrayBase, Dim, OwnedRepr};
use pyo3::{
    prelude::*, types::PyDict
};

static COUNTER: AtomicU16 = AtomicU16::new(1);

#[allow(dead_code)]
#[derive(Debug)]
#[pyclass(
    name = "Env", 
    set_all, 
    subclass, 
    get_all, 
    dict, 
    sequence, 
    // unsendable
)]

pub struct Env {
    id: u16,
    name: String,
    args : PyObject ,
    kwargs : PyObject,
}

#[pymethods]
impl Env {
    #[new]
    #[pyo3(signature = ( name, *args, **kwargs)
        ,text_signature = "( name : str,  *args, **kwarg)"
        )]
    pub fn new<'py>(
        _py : Python<'_>,
        name: String,
        args: &Bound<'_, PyAny>,
        kwargs: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let current: u16 = COUNTER.load(Ordering::SeqCst).into();
        COUNTER.fetch_add(1, Ordering::SeqCst);

        // let ac_bind : &PyArrayDyn<i64> = action_space.extract(py).expect("action extraction failed") ;
        // let ob_bind : &PyArrayDyn<i64> = observation_space.extract(py).expect("observation extraction failed") ;
        // let ac_type = ac_bind.get_type().to_string();
        // let ob_type = ob_bind.get_type().to_string();
        // // run_py(&action_space, "type(value)");

        // let ob_type: String = run_py(&observation_space, "type(value)");

        // let ac: PyArrayDyn<PyObject> = match ac_type.as_str() {
        //     "<class 'list'>" => action_space.extract(py).expect("sction casting faild"),
        //     "<class 'numpy.ndarray'>" => action_space,

        //     _ => {
        //         return Err(PyErr::new::<PyValueError, _>(format!(
        //             "Invalid type of action space type {}, Only `list` or `numpy.ndarray`",
        //             ac_type
        //         )))
        //     }
        // };
        // let env_space: PyArrayDyn<PyObject> = match ob_type.as_str() {
        //     "<class 'numpy.ndarray'>" => observation_space,
        //     _ => {
        //         return Err(PyErr::new::<PyValueError, _>(format!(
        //             "Invalid type of env space {} , only `numpy.ndarray` is acceptable",
        //             ob_type
        //         )))
        //     }
        // };
        Ok(Self {
            id: current,
            name,
            args : args.clone().unbind() ,
            kwargs : kwargs.unwrap().clone().unbind()
        })
    }

    pub fn reset(&mut self) {}
    ///# step
    /// ## input
    ///  state
    /// #### action
    /// ## output
    ///    reward
    ///     done
    ///     etc
    #[pyo3(text_signature = "($cls, _state, _action)")]
    pub fn step<'py>(
        &mut self,
        py: Python<'py>,
        _state: PyObject,
        _action: u8,
    ) -> PyResult<(PyObject, bool, &'py PyDict)> {
        let reward = 0.0.to_object(py);
        let info = PyDict::new_bound(py)
                                        .unbind()
                                        .extract(py)?;
        let done: bool = false;

        Ok((reward, done, info))
    }

    pub fn close(&mut self) {}

    fn __str__(slf: &PyBound<Self>,  _py: Python<'_> ) -> String {
        let class_name = slf.get_type().qualname().unwrap();
        format!("{}(id = {}, name = {})", 
        class_name,
         slf.borrow().id,
        slf.borrow().name
        )
        // let obsev: &PyArrayDyn<PyObject> = slf.borrow().observation_space.extract(py).expect("extract Failed");
        // let ac: &PyArrayDyn<PyObject> = slf.borrow().action_space.extract(py).expect("extract Failed");

        // let observation_space_size = obsev .shape() ;
        // let action_space_size = ac.shape();

        // format!(
        //     "{}(name: {}, action space size : {:?}, envirnment shape {:?})",
        //     class_name, slf.borrow().name, action_space_size, observation_space_size
        // )
    }
    fn __repr__(slf: &PyBound<Self>) -> PyResult<String> {
        let class_name = slf.get_type().qualname().unwrap();
        Ok(format!("{}", class_name))
    }
}


