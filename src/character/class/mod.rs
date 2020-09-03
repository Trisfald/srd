//! Character classes.

pub mod fighter;
pub use self::fighter::FIGHTER;

use crate::character::level::Level;
use crate::hit_points::HitDice;
use crate::proficiency::ProficiencyBonus;
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
        Self(item.to_string())
    }
}

/// Describes all bonuses and maluses of a class.
pub trait ClassModel {
    /// Returns the hit dice per level.
    fn hit_dice(&self) -> HitDice;

    /// Returns the base hit points at first level.
    fn hit_points_at_1st_level(&self) -> u8 {
        self.hit_dice().die.max()
    }

    /// Returns the proficiency bonus at the given level.
    fn proficiency_bonus(&self, level: &Level) -> ProficiencyBonus {
        proficiency_bonus_at_level(level)
    }
}

/// Returns the standard proficiency bonus at the given level.
const fn proficiency_bonus_at_level(level: &Level) -> ProficiencyBonus {
    ProficiencyBonus((level.value() - 1) / 4 + 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_proficiency_bonus() {
        macro_rules! test_case {
            ($lvl:expr, $expected:expr ) => {
                assert_eq!(
                    proficiency_bonus_at_level(&Level::new($lvl).unwrap()),
                    $expected.into()
                );
            };
        }
        test_case!(1, 2);
        test_case!(2, 2);
        test_case!(3, 2);
        test_case!(4, 2);
        test_case!(5, 3);
        test_case!(6, 3);
        test_case!(7, 3);
        test_case!(8, 3);
        test_case!(9, 4);
        test_case!(10, 4);
        test_case!(11, 4);
        test_case!(12, 4);
        test_case!(13, 5);
        test_case!(14, 5);
        test_case!(15, 5);
        test_case!(16, 5);
        test_case!(17, 6);
        test_case!(18, 6);
        test_case!(19, 6);
        test_case!(20, 6);
    }
}
