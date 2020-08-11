//! Character races.

pub mod hill_dwarf;
pub use self::hill_dwarf::HILL_DWARF;

use serde::{Deserialize, Serialize};

/// Number of reserved races (zero indexed).
pub const RESERVED_RACES: u8 = 8;

/// Identifies a race.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct RaceId(pub u8);

/// Describes all bonuses and maluses of a race.
pub trait RaceModel {}
