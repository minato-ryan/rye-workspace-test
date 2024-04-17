use pyo3::prelude::*;

/// Prints a message.
#[pyfunction]
fn hello(name: String) -> String {
    "hello ".to_owned() + &name
}

/// A Python module implemented in Rust.
#[pymodule]
fn _lowlevel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}