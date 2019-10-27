## Blog Post: Extending Python with Rust

This repository contains the code supporting the blog post.

Read [this blog post](https://medium.com/@p_chhetri/extending-python-with-rust-84e9299d34c1) for more details.


# Install

- Python `pip install pytest-benchmark`


# Compile

- [Rust](https://www.rust-lang.org) `cargo build --release` compiles to `libmyrustlib.so`.
- [Nim](https://nim-lang.org) `nim c -d:release pyext-mynimlib/src/libmynimlib.nim` compiles to `libmynimlib.so`.


# Test

- [Nim](https://nim-lang.org) `nim c -r -d:release pyext-mynimlib/tests/test1.nim`.
- [Nim](https://nim-lang.org) Performance Test `nim c -r -d:release pyext-mynimlib/benchmarks/benchmark.nim`.
