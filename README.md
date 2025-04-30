# fstest
[![Crates.io Version](https://img.shields.io/crates/v/fstest)
![Crates.io Total Downloads](https://img.shields.io/crates/d/fstest)](https://crates.io/crates/fstest)
[![🦀 Continuous Integration](https://github.com/JensKrumsieck/fstest/actions/workflows/ci.yaml/badge.svg)](https://github.com/JensKrumsieck/fstest/actions/workflows/ci.yaml)
[![Docs.rs](https://img.shields.io/docsrs/fstest/latest)](https://docs.rs/fstest)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/jenskrumsieck/fstest#license)

`fstest` provides a procedural macro attribute for simplifying integration tests involving
temporary file system setups and optional Git repository initialization.
This crate defines the `#[fstest]` macro, which wraps a test function and handles:
- Creation of a temporary working directory
- Optional initialization of a Git repository in the temporary directory
- Copying of specified input files into the temp directory
- Restoring the original working directory after test execution

## Usage
```rust
use fstest::cmd_test;
#[cmd_test(repo = true, files = ["tests/data/input.txt", "tests/data/config.toml"])] //arguments are optional!
fn my_test(tempdir: &std::path::Path) {
    // test code working within `tempdir`
}
```
See examples folder for examples.

## Installation
Add crate and serial_test and tempfile to your dev dependencies.
```toml
[dev-dependencies]
fstest = "0.1.0"
tempfile = "3"
serial_test = "3"
```