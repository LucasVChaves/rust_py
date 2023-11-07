use pyo3::prelude::*;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn count_words(s: String) -> Py<PyAny> {
    let mut hm = HashMap::new();
    for sub in s.split(' '){
        let count = hm
            .entry(sub)
            .or_insert(0);
        *count += 1;
    }
    
    return Python::with_gil(|py| {
        hm.to_object(py)
    });
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}
