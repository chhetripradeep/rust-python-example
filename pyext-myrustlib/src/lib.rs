#[macro_use] extern crate cpython;

use cpython::{PyResult, Python};

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_libmyrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "rust_concat", py_fn!(py, rust_concat(val: i64)))?;
    Ok(())
});

fn rust_concat(_: Python, val: i64) -> PyResult<String> {
    let mut result = String::new();
    for i in 0..val {
        result.push_str(&i.to_string());
    }
    Ok(result)
}