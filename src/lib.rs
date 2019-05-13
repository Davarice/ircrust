extern crate pyo3;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use pyo3::wrap_pyfunction;
use std::str;

/// Split a raw IRCv3 line into a usable Dict.
#[pyfunction]
fn decode(input: &PyBytes) -> PyResult<PyObject> {
    // First, decode the data into something we can work.
    let raw_str: &str = str::from_utf8(input.as_bytes())?;

    // Then, initialize the Dict.
    let gil: GILGuard = Python::acquire_gil();
    let py: Python = gil.python();
    let tags_dict: &PyDict = PyDict::new(py);
    let mut tag_str: &str;
    let mut msg_str: &str;

    // Third, break the line down.
    if raw_str.starts_with('@') {
        // The Tags String is the first half of the original message received by IRC. The "regular"
        //  message begins after the first space.
        // Find the first space.
        let idx = raw_str.find(' ');
        if idx == None {
            // There is no space. The entire line after "@" is nothing but tags. Weird but okay.
            tag_str = &raw_str[1..];
        } else {
            // Found the space. Before it (and after "@") is tags, after is message.
            tag_str = &raw_str[1..idx.unwrap()];
            msg_str = &raw_str[idx.unwrap() + 1..];
        }
        // Break the tagstr into a Vector.
        let tags_str_vec: Vec<&str> = tag_str.split(';').collect();

        // Loop through the vector of pair strings, and break each one the rest of the way down. Add
        //  values to the Dict.
        for &kvp in tags_str_vec.iter() {
            let mut key: &str;
            let mut val: &str;
            if kvp.contains('=') {
                // If the key has an `=`, the text to the right is the value.
                let idx = kvp.find('=').unwrap();
                key = &kvp[..idx];
                val = &kvp[idx + 1..];
            } else {
                // Otherwise, the value is to be interpreted as empty.
                key = kvp;
                val = "";
            }
            tags_dict.set_item(key, val);
        }
    } else {
        // There are no tags. This is pure message.
        msg_str = raw_str
    }

//    let output: (str, str, str, PyObject) = (hostname, command, text, tags_dict.into());

    Ok(tags_dict.into())
}

/// A module for manipulation of IRCv3 data.
#[pymodule]
fn ircsplit(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(decode))?;

    Ok(())
}
