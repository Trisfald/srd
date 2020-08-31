//! Hit Points of creatures.

use crate::character::level::LEVEL_MAX;
use crate::dice::Dice;
use crate::error::{SRDError, SRDResult};
use crate::value::PositiveBoundedValue;
use serde::{Deserialize, Serialize};

/// Represents the health of a creature.
///
/// # Reference
///
/// Hit points represent a combination of physical and mental durability, the will to live, and luck.\
/// Creatures with more hit points are more difficult to kill. Those with fewer hit points are more fragile.\
/// A creature's current hit points (usually just called hit points) can be any number from the creature's
/// hit point maximum down to 0. This number changes frequently as a creature takes damage or receives healing.
pub type HitPoints = PositiveBoundedValue<u16>;

/// The dice used to generate a creature's hit points.
///
/// # Reference
///
/// You add together the Hit Dice granted by all your classes to form your pool of Hit Dice.
pub type HitDice = Dice;

/// This struct contains the history of all hit points rolls for a particular creature.
#[derive(Default, Clone, Serialize, Deserialize)]
pub(crate) struct HitPointsHistory {
    results: [u8; LEVEL_MAX as usize],
}

impl HitPointsHistory {
    /// Returns how many hit points rolls are contained in this struct.
    pub(crate) fn count(&self) -> usize {
        self.results
            .iter()
            .position(|n| *n == 0)
            .unwrap_or(LEVEL_MAX as usize)
    }

    /// Returns the total sum of hit points.
    pub(crate) fn total(&self) -> u16 {
        self.results[0..self.count()]
            .iter()
            .fold(0, |acc, n| acc + *n as u16)
    }

    /// Adds one hit points roll result.
    pub(crate) fn add_result(&mut self, result: u8) -> SRDResult<()> {
        if result == 0 {
            return Err(SRDError::InvalidArgument(format!(
                "hit points per level can't be {}",
                result
            )));
        }
        let count = self.count();
        if count >= LEVEL_MAX as usize {
            Err(SRDError::MaxLevelReached)
        } else {
            self.results[self.count()] = result;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hp_history_initialization() {
        let hp = HitPointsHistory::default();
        assert_eq!(hp.count(), 0);
        assert_eq!(hp.total(), 0);
    }

    #[test]
    fn hp_history_add_result() {
        let mut hp = HitPointsHistory::default();
        for i in 1..=20 {
            assert!(hp.add_result(3).is_ok());
            assert_eq!(hp.count(), i);
            assert_eq!(hp.total() as usize, i * 3);
        }
        assert_eq!(hp.add_result(3).err(), Some(SRDError::MaxLevelReached));
    }

    #[test]
    fn hp_history_add_result_invalid() {
        let mut hp = HitPointsHistory::default();
        assert!(hp.add_result(0).is_err());
    }
}
