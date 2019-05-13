extern crate pyo3;

mod subsplit;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use pyo3::wrap_pyfunction;
use std::str;
use subsplit::split_at_first;

/// Split a raw IRCv3 line into a usable Dict.
#[pyfunction]
fn decode(input: &PyBytes) -> PyResult<PyObject> {
    // First, decode the data into something we can work.
    let raw_str: &str = str::from_utf8(input.as_bytes())?;

    // Initialize the variables that will be used during parsing.
    let mut message: &str;
    let prefix: &str;
    let command: &str;
    let trail: &str;

    // Then, initialize the Output Structures.
    let gil: GILGuard = Python::acquire_gil();
    let py: Python = gil.python();
    let tags_dict: &PyDict = PyDict::new(py);
    let mut output: (&str, &str, &str, PyObject);

    // Third, break the line down.
    if raw_str.starts_with('@') {
        // The Tags String is the first half of the original message received by IRC. The "regular"
        //  message begins after the first space.
        let (tag_str, msg_str) = split_at_first(&raw_str[1..], ' ');
        message = msg_str;

        // Break the tagstr into a Vector.
        let tags_str_vec: Vec<&str> = tag_str.split(';').collect();

        // Loop through the vector of pair strings, and break each one the rest of the way down. Add
        //  values to the Dict.
        for &kvp in tags_str_vec.iter() {
            if !kvp.is_empty() {
                let (key, val) = split_at_first(kvp, '=');
                if !key.is_empty() {
                    tags_dict.set_item(key, val)?;
                }
            }
        }
    } else {
        // There are no tags. This is pure message.
        message = raw_str;
    }

    let prefix: &str;
    let command: &str;
    let trail: &str;

    // Now, parse the message itself.
    // This format is specified in Section 2.3.1 of RFC 1459.
    if message.starts_with(':') {
        // This Message has a Prefix. The Prefix is most likely hostname and/or server info.
        let (a, b) = split_at_first(message, ':');
        prefix = a;
        message = b;
    }

    Ok(tags_dict.into())
}

/// A module for manipulation of IRCv3 data.
#[pymodule]
fn ircsplit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(decode))?;

    Ok(())
}
