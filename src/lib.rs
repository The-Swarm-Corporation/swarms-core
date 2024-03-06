use pyo3::prelude::*;
use pyo3::types::PyAny;
use rayon::prelude::*;

#[pyfunction]
fn execute_in_parallel(py: Python, py_func: PyObject, n: usize) -> PyResult<Vec<PyObject>> {
    // Convert PyObject to Py<PyAny> for safe cross-thread usage
    let py_func: Py<PyAny> = py_func.extract(py)?;

    // Use Rayon to execute the Python function in parallel
    let results: Result<Vec<_>, _> = (0..n).into_par_iter()
        .map(|_| {
            Python::with_gil(|py| {
                // Safely call the Python function and convert the result back to PyObject
                py_func.call1(py, ()).map(|res| res.to_object(py))
            })
        })
        .collect();

    results.map_err(|e| e.into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyrust_parallel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(execute_in_parallel, m)?)?;
    Ok(())
}
