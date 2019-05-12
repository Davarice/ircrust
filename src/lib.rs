use pyo3::types::{PyBytes, PyDict, PyByteArray};

fn main() {
    extern crate pyo3;
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;

    /// Split a raw IRCv3 line into a usable Dict.
    #[pyfunction]
    fn decode(line: &str) -> PyResult<PyObject> {
        // First, initialize the Dict.
        let gil = Python::acquire_gil();
        let py: Python = gil.python();
        let dict: &PyDict = PyDict::new(py);
        // Then, break the line down.
        // TODO: Break apart the String and fill the Dict.
        for i in 0..line.len() {
            dict.set_item(i, i + 2);
        }
        Ok(dict.into())
    }

    /// A module for manipulation of IRCv3 data.
    #[pymodule]
    fn ircsplit(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_wrapped(wrap_pyfunction!(decode))?;

        Ok(())
    }
}
