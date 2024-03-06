#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::types::IntoPyDict;

    #[test]
    fn test_execute_in_parallel() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let py_func = py
            .run(
                "def test_func(): return 42",
                None,
                Some(py.import("builtins").unwrap().into_py_dict(py)),
            )
            .unwrap()
            .extract::<PyObject>(py)
            .unwrap();

        let result = execute_in_parallel(py, py_func, 10).unwrap();
        assert_eq!(result.len(), 10);
        for res in result {
            let res: i32 = res.extract(py).unwrap();
            assert_eq!(res, 42);
        }
    }

    #[test]
    fn test_execute_in_parallel_with_error() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let py_func = py
            .run(
                "def test_func(): raise ValueError('test error')",
                None,
                Some(py.import("builtins").unwrap().into_py_dict(py)),
            )
            .unwrap()
            .extract::<PyObject>(py)
            .unwrap();

        let result = execute_in_parallel(py, py_func, 10);
        assert!(result.is_err());
    }
}