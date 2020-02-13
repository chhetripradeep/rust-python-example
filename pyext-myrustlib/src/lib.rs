#[macro_use] extern crate cpython;

use std::fmt::Write;
use cpython::{PyResult, Python};

py_module_initializer!(libmyrustlib, initlibmyrustlib, PyInit_libmyrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "rust_concat", py_fn!(py, rust_concat(val: i64)))?;
    m.add(py, "rust_concat_reserve", py_fn!(py, rust_concat_reserve(val: i64)))?;
    m.add(py, "rust_concat_logdigits", py_fn!(py, rust_concat_logdigits(val: i64)))?;
    m.add(py, "rust_concat_buffer", py_fn!(py, rust_concat_buffer(val: i64)))?;
    m.add(py, "rust_concat_inplace", py_fn!(py, rust_concat_inplace(val: i64)))?;
    Ok(())
});

fn rust_concat(_: Python, val: i64) -> PyResult<String> {
    let mut result = String::new();
    for i in 0..val {
        result.push_str(&i.to_string());
    }
    Ok(result)
}

fn rust_concat_reserve(_: Python, val: i64) -> PyResult<String> {
    let mut digits = 0usize;
    let mut order = 1usize;
    for i in 0usize..(val as usize) {
        if i == order * 10 {
            order *= 10
        }
        digits += order;
    }

    let mut result = String::with_capacity(digits);
    for i in 0usize..(val as usize) {
        result.push_str(&i.to_string());
    }
    Ok(result)
}

fn range_digits(val: i64) -> usize {
    assert!(val >= 0);
    if val == 0 {
        return 0;
    }
    let val = val as usize;
    let mut digits = 1usize;
    let mut base = 1usize;
    let mut digits_sum = 1usize; // For 0.
    while 10usize * base < val {
        digits_sum += 9usize * digits * base;
        digits += 1usize;
        base *= 10usize;
    }
    digits_sum += (val - base) * digits;
    digits_sum
}

fn rust_concat_logdigits(_: Python, val: i64) -> PyResult<String> {
    let digits = range_digits(val);
    let mut result = String::with_capacity(digits);
    for i in 0usize..(val as usize) {
        result.push_str(&i.to_string());
    }
    Ok(result)
}

fn rust_concat_buffer(_: Python, val: i64) -> PyResult<String> {
    let digits = range_digits(val);
    let mut result = String::with_capacity(digits);
    let mut tmp = val.to_string();
    for i in 0usize..(val as usize) {
        tmp.clear();
        write!(&mut tmp, "{}", i).unwrap();
        result.push_str(&tmp);
    }
    Ok(result)
}

fn rust_concat_inplace(_: Python, val: i64) -> PyResult<String> {
    let digits = range_digits(val);
    let mut result = String::with_capacity(digits);
    for i in 0usize..(val as usize) {
        write!(&mut result, "{}", i).unwrap();
    }
    Ok(result)
}
