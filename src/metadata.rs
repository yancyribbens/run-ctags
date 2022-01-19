use serde::{Deserialize, Serialize};

use std::error::Error;
use std::process::Command;

use crate::package::Package;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub packages: Vec<Package>,
}

impl Metadata {
    /// Executes "cargo metadata --format-version=1".
    ///
    /// Cargo Metadata returns json which is then serialized to
    /// a Rust struct using serde which magically transforms
    /// our json returned by Cargo to a Rust struct with the
    /// matching key values:
    ///
    /// Metadata { packages:
    ///     [ Package { manifest_path: "/path/to/manifest" } ]
    /// }

    pub fn fetch() -> Result<Metadata, Box<dyn Error>> {
        let output = Command::new("cargo")
            .arg("metadata")
            .arg("--format-version=1")
            .output()?;

        let metadata: String = std::str::from_utf8(&output.stdout)?.to_string();
        let m: Metadata = serde_json::from_str(&metadata)?;

        Ok(m)
    }
}
