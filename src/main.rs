//! # run-ctags
//!
//! A utility for finding all Rust dependencies with Cargo.
use cargo_metadata::Metadata;
use cargo_metadata::MetadataCommand;

/// For each package, write the package path to stdout separated by a newline.
pub fn main() {
    let mut m: Metadata = MetadataCommand::new()
        .manifest_path("./Cargo.toml")
        .exec()
        .unwrap();

    for p in &mut m.packages {
        p.manifest_path.pop();
        println!("{}", p.manifest_path);
    }
}
