use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use scryer_prolog::machine::Machine;
use send_wrapper::SendWrapper;

#[pyclass]
pub struct PrologEngine {
    machine: SendWrapper<Machine>,
}

#[pymethods]
impl PrologEngine {
    #[new]
    fn new() -> Self {
        PrologEngine {
            machine: SendWrapper::new(Machine::new_lib()),
        }
    }

    fn load_module(&mut self, module_name: &str, module_code: String) -> PyResult<()> {
        let mut machine = &mut self.machine; //.deref_mut();
        machine.load_module_string(module_name, module_code);
        Ok(())
    }

    fn run_query(&mut self, query: String) -> PyResult<PyObject> {
        let mut machine = &mut self.machine; //.deref_mut();
        let output = machine.run_query(query).unwrap().to_string();
        Python::with_gil(|py| {
            let python_json = PyModule::import(py, "json")?;
            let result_dict: PyObject = python_json
                .getattr("loads")?
                .call1((output,))?
                .to_object(py);
            Ok(result_dict.into())
        })
    }
}

#[pymodule]
#[pyo3(name = "pyscryer")]
fn pyscryer(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PrologEngine>()?;

    Ok(())
}
