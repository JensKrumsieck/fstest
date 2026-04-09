//! # fstest
//! `fstest` provides a procedural macro attribute for simplifying integration tests involving
//! temporary file system setups and optional Git repository initialization.
//! This crate defines the `#[fstest]` macro, which wraps a test function and handles:
//! - Creation of a temporary working directory
//! - Optional initialization of a Git repository in the temporary directory
//! - Copying of specified input files into the temp directory
//! - Restoring the original working directory after test execution
//! ## Usage
//!
//! ```rust
//! use fstest::fstest;
//! use std::path::Path;
//! 
//! #[fstest(repo = true, files = ["tests/data/input.txt", "tests/data/config.toml"])]
//! fn my_test(tempdir: &Path) {
//!     // test code working within `tempdir`
//! }
//! ```
pub use serial_test;
pub use tempfile;
pub use fstest_macro::fstest;

use git2::{IndexAddOption, Repository};
use std::{iter, path::Path};

/// Creates a new git repository in the given directory and makes an initial commit with all files in the repository.
/// If there is no user configured, it sets a default user name and email.
/// This function is used for setting up a new git repository for testing purposes.
pub fn create_repo_and_commit(dir: &Path) -> Result<(), git2::Error> {
    // create repo:
    let repo = Repository::init(dir)?;
    //stage all
    let mut index = repo.index()?;
    index.add_all(iter::once(&"*"), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    if repo.signature().is_err() {
        let mut cfg = repo.config()?;
        cfg.set_str("user.name", "Derp")?;
        cfg.set_str("user.email", "derp@google.de")?;
    }

    //commit
    let new_oid = index.write_tree()?;
    let new_tree = repo.find_tree(new_oid)?;
    let author = repo.signature()?;
    repo.commit(
        Some("HEAD"),
        &author,
        &author,
        "Initial Commit",
        &new_tree,
        &[],
    )?;

    Ok(())
}
