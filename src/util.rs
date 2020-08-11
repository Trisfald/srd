//! Utility functions.

use crate::constants::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
/// Represents the version of this package as specified in `Cargo.toml`.
pub(crate) struct PackageVersion {
    pub(crate) major: u16,
    pub(crate) minor: u16,
    pub(crate) patch: u16,
}

impl PackageVersion {
    pub(crate) const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Constructs a PackageVersion from the environmental variables exposed by cargo.
    pub(crate) fn from_env() -> Self {
        let major = VERSION_MAJOR.parse().unwrap();
        let minor = VERSION_MINOR.parse().unwrap();
        let patch = VERSION_PATCH.parse().unwrap();
        Self::new(major, minor, patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_version_from_env() {
        let version = PackageVersion::from_env();
        assert_ne!(version, PackageVersion::new(0, 0, 0));
    }
}
