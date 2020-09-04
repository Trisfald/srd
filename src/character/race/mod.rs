//! Character races.

pub mod dwarf;

pub mod generic_model;
pub use self::generic_model::GenericRaceModel;

pub mod hill_dwarf;
pub use self::hill_dwarf::HILL_DWARF;

use crate::ability::{AbilityId, AbilityScore};
use serde::{Deserialize, Serialize};

/// Number of core races.
pub const RESERVED_RACES: u8 = 8;

/// Identifies a race.
///
/// Races from the SRD starts with an `_`.
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct RaceId(pub String);

impl From<&str> for RaceId {
    fn from(item: &str) -> Self {
        Self(item.to_string())
    }
}

/// Describes all bonuses and maluses of a race.
pub trait RaceModel {
    /// Returns the list of ability score increase.
    fn ability_score_increase(&self) -> &[(AbilityId, AbilityScore)] {
        &[]
    }
}
