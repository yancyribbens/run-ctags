//! # run-ctags
//!
//! A utility for finding all Rust dependencies with Cargo.

/// Fetch the project metadata.
pub mod metadata;

/// A single dependency for the project.
pub mod package;

use crate::metadata::Metadata;

/// For each package, write the package path to stdout separated by a newline.
pub fn main() {
    let metadata = Metadata::fetch().unwrap();

    for package in metadata.packages {
        let folder = package.root();
        println!("{}", folder.into_string().unwrap());
    }
}
