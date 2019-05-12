extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use pyo3::wrap_pyfunction;
use std::str;

/// Split a raw IRCv3 line into a usable Dict.
#[pyfunction]
fn decode(input: &PyBytes) -> PyResult<PyObject> {
    // First, decode the data into something we can work.
    let line: &str = match str::from_utf8(input.as_bytes()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    // Then, initialize the Dict.
    let gil = Python::acquire_gil();
    let py: Python = gil.python();
    let dict: &PyDict = PyDict::new(py);

    // Third, break the line down.
    // TODO: Break apart the String.

    // Finally, populate the Dict with all the values.
    for i in 0..line.len() {
        // FIXME: Placeholder routine until String breaking is ready.
        dict.set_item(i, line)?;
    }

    Ok(dict.into())
}

/// A module for manipulation of IRCv3 data.
#[pymodule]
fn ircsplit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(decode))?;

    Ok(())
}
