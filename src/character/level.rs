//! Levels of characters.

use crate::error::{SRDError, SRDResult};
use serde::{Deserialize, Serialize};

/// Minimum level.
const LEVEL_MIN: u8 = 1;

/// Maximum level.
const LEVEL_MAX: u8 = 20;

/// Character's level.
///
/// # Reference
///
/// As your character goes on adventures and overcomes challenges, he or she gains experience,
/// represented by experience points. A character who reaches a specified experience point total
/// advances in capability. This advancement is called gaining a level.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Level {
    value: u8,
}

impl Level {
    /// Returns true if the given value is a valid level.
    pub fn is_value_valid(value: u8) -> bool {
        crate::util::is_value_valid(value, LEVEL_MIN, LEVEL_MAX)
    }

    /// Constructs a new `Level` with the given value.
    ///
    /// # Errors
    ///
    /// An error is returned if the provided value is out of bounds.
    pub fn new(value: u8) -> SRDResult<Self> {
        if Self::is_value_valid(value) {
            Ok(Self { value })
        } else {
            Err(SRDError::InvalidLevel(value))
        }
    }

    /// Returns the value of this level.
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Sets a new value.
    ///
    /// # Errors
    ///
    /// An error is returned if the provided value is out of bounds.    
    pub fn set_value(&mut self, value: u8) -> SRDResult<()> {
        if Self::is_value_valid(value) {
            self.value = value;
            Ok(())
        } else {
            Err(SRDError::InvalidLevel(value))
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self { value: 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_new_has_bounds() {
        assert!(Level::new(3).is_ok());
        assert!(Level::new(LEVEL_MIN - 1).is_err());
        assert!(Level::new(LEVEL_MAX + 1).is_err());
    }

    #[test]
    fn level_set_has_bounds() {
        let mut level = Level::new(3).unwrap();
        assert!(level.set_value(LEVEL_MIN - 1).is_err());
        assert!(level.set_value(LEVEL_MAX + 1).is_err());
        assert!(level.set_value(2).is_ok());
        assert_eq!(level.value(), 2);
    }
}
