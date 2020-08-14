//! This module contains creatures' abilities.

use crate::{SRDError, SRDResult};
use serde::{Deserialize, Serialize};

/// Identifies an ability.
///
/// # Reference
///
/// An ability provides a quick description of a creature's physical or mental characteristic.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct AbilityId(pub u8);

impl From<u8> for AbilityId {
    fn from(item: u8) -> Self {
        AbilityId(item)
    }
}

/// Id of the Strength ability.
///
/// # Reference
///
/// Strength measures physical power.
pub const STRENGTH: AbilityId = AbilityId(0);

/// Id of the Dexterity ability.
///
/// # Reference
///
/// Dexterity measures agility.
pub const DEXTERITY: AbilityId = AbilityId(1);

/// Id of the Constitution ability.
///
/// # Reference
///
/// Constitution measures endurance.
pub const CONSTITUTION: AbilityId = AbilityId(2);

/// Id of the Intelligence ability.
///
/// # Reference
///
/// Intelligence measures reasoning and memory.
pub const INTELLIGENCE: AbilityId = AbilityId(3);

/// Id of the Wisdom ability.
///
/// # Reference
///
/// Wisdom measures perception and insight.
pub const WISDOM: AbilityId = AbilityId(4);

/// Id of the Charisma ability.
///
/// # Reference
///
/// Charisma measures force of personality.
pub const CHARISMA: AbilityId = AbilityId(5);

/// Number of core abilities.
pub const RESERVED_ABILITIES: u8 = 6;

const ABILITY_SCORE_MIN: u8 = 1;

const ABILITY_SCORE_MAX: u8 = 30;

/// The numeric value of an ability.
///
/// # Reference
///
/// Each of a creature's abilities has a score, a number that defines the magnitude of that
/// ability.
///
/// Each ability also has a modifier, derived from the score and ranging from −5
/// (for an ability score of 1) to +10 (for a score of 30).
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AbilityScore {
    value: u8,
}

impl AbilityScore {
    /// Returns true if the given value is a valid ability score.
    pub fn is_value_valid(value: u8) -> bool {
        value >= ABILITY_SCORE_MIN && value <= ABILITY_SCORE_MAX
    }

    /// Creates a new `AbilityScore` with the given value.
    ///
    /// # Errors
    ///
    /// An error is returned if the provided value is out of bounds.
    pub fn new(value: u8) -> SRDResult<Self> {
        if Self::is_value_valid(value) {
            Ok(Self { value })
        } else {
            Err(SRDError::InvalidAbilityScore(value))
        }
    }

    /// Creates a new `AbilityScore` capped between the min and max value.
    pub fn capped(value: u8) -> Self {
        if value > ABILITY_SCORE_MAX {
            Self {
                value: ABILITY_SCORE_MAX,
            }
        } else if value < ABILITY_SCORE_MIN {
            Self {
                value: ABILITY_SCORE_MIN,
            }
        } else {
            Self { value }
        }
    }

    /// Adds `value` to the current ability score's value, respecting the bounds.
    pub fn add_with_cap(&mut self, value: u8) {
        let new_value = self.value + value;
        if new_value > ABILITY_SCORE_MAX {
            self.value = ABILITY_SCORE_MAX;
        } else {
            self.value = new_value;
        }
    }

    /// Returns the value of this ability score.
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
            Err(SRDError::InvalidAbilityScore(value))
        }
    }

    /// Returns the modifier of this ability score.
    ///
    /// # Reference
    ///
    /// To determine an ability modifier without consulting the table, subtract 10 from
    /// the ability score and then divide the total by 2 (round down).
    pub fn modifier(&self) -> i8 {
        ((self.value as i8 + 10) / 2) - 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ability_score_new_has_bounds() {
        assert!(AbilityScore::new(3).is_ok());
        assert!(AbilityScore::new(ABILITY_SCORE_MIN - 1).is_err());
        assert!(AbilityScore::new(ABILITY_SCORE_MAX + 1).is_err());
    }

    #[test]
    fn ability_score_capped_respects_bounds() {
        assert_eq!(
            AbilityScore::capped(ABILITY_SCORE_MIN - 1).value(),
            ABILITY_SCORE_MIN
        );
        assert_eq!(
            AbilityScore::capped(ABILITY_SCORE_MAX + 1).value(),
            ABILITY_SCORE_MAX
        );
    }

    #[test]
    fn ability_score_add_with_cap_respects_bounds() {
        let mut ability = AbilityScore::capped(3);
        ability.add_with_cap(ABILITY_SCORE_MAX);
        assert_eq!(ability.value(), ABILITY_SCORE_MAX);
    }

    #[test]
    fn ability_score_set_has_bounds() {
        let mut ability = AbilityScore::capped(3);
        assert!(ability.set_value(ABILITY_SCORE_MIN - 1).is_err());
        assert!(ability.set_value(ABILITY_SCORE_MAX + 1).is_err());
        assert!(ability.set_value(2).is_ok());
        assert_eq!(ability.value(), 2);
    }

    #[test]
    fn ability_score_modifier() {
        assert_eq!(AbilityScore::capped(1).modifier(), -5);
        assert_eq!(AbilityScore::capped(4).modifier(), -3);
        assert_eq!(AbilityScore::capped(9).modifier(), -1);
        assert_eq!(AbilityScore::capped(10).modifier(), 0);
        assert_eq!(AbilityScore::capped(15).modifier(), 2);
        assert_eq!(AbilityScore::capped(24).modifier(), 7);
        assert_eq!(AbilityScore::capped(30).modifier(), 10);
    }
}
