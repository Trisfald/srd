//! Character classes.

pub mod fighter;
pub use self::fighter::FIGHTER;

use serde::{Deserialize, Serialize};

/// Number of core classes.
pub const RESERVED_CLASSES: u8 = 12;

/// Identifies a class.
///
/// Classes from the SRD starts with an `_`.
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct ClassId(pub String);

impl From<&str> for ClassId {
    fn from(item: &str) -> Self {
        ClassId(item.to_string())
    }
}

/// Describes all bonuses and maluses of a class.
pub trait ClassModel {}
