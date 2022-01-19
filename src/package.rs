use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::PathBuf;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    pub manifest_path: String,
}

impl Package {
    /// The project root directory is the directory
    /// that contains the Cargo.toml file.  The Cargo.toml
    /// file location is the manifest_path location, therefore,
    /// the Cargo.toml file is removed and the remaining
    /// parent path is returned.
    ///
    /// example:
    ///
    /// manifest_path: "/parent/project/Cargo.toml"
    ///
    /// root: "/parent/project"
    ///

    pub fn root(&self) -> OsString {
        let mut path = PathBuf::from(&self.manifest_path);
        path.pop();
        path.into_os_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_package_root() {
        let package = Package {
            manifest_path: String::from("/parent/path/Cargo.toml"),
        };

        let path = package.root().into_string().unwrap();

        assert_eq!(String::from("/parent/path"), path);
    }
}
