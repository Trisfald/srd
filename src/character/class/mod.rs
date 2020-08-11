//! Character classes.

pub mod fighter;
pub use self::fighter::FIGHTER;

use serde::{Deserialize, Serialize};

/// Number of reserved classes (zero indexed).
pub const RESERVED_CLASSES: u8 = 12;

/// Identifies a class.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct ClassId(pub u8);

/// Describes all bonuses and maluses of a class.
pub trait ClassModel {}
