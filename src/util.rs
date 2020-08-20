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
    #[allow(dead_code)] // TODO remove as soon as this's used
    pub(crate) const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Constructs a new `PackageVersion` from the environmental variables exposed by cargo.
    #[allow(dead_code)] // TODO remove as soon as this's used
    pub(crate) fn from_env() -> Self {
        let major = VERSION_MAJOR.parse().unwrap();
        let minor = VERSION_MINOR.parse().unwrap();
        let patch = VERSION_PATCH.parse().unwrap();
        Self::new(major, minor, patch)
    }
}

pub(crate) fn is_value_valid<T: PartialOrd>(value: T, min: T, max: T) -> bool {
    assert!(min <= max);
    value >= min && value <= max
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
