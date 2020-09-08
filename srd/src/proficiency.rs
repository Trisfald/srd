//! Creatures proficiencies.

use serde::{Deserialize, Serialize};

/// Tells whether or not a proficiency is known.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Proficiency(pub bool);

impl From<bool> for Proficiency {
    fn from(item: bool) -> Self {
        Self(item)
    }
}

/// Default proficiency (value is false).
pub(crate) static DEFAULT_PROFICIENCY: Proficiency = Proficiency(false);

/// Proficiency bonus.
///
/// # Reference
///
/// Characters have a proficiency bonus determined by level. Monsters also have this bonus.\
/// Your proficiency bonus can't be added to a single die roll or other number more than once.\
/// Occasionally, your proficiency bonus might be multiplied or divided.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ProficiencyBonus(pub u8);

impl ProficiencyBonus {
    /// Multiplies this object by `n` and returns the result in a new `ProficiencyBonus`.
    pub const fn multiply(&self, n: u8) -> Self {
        let Self(value) = self;
        Self(*value * n)
    }

    /// Divides this object by `n` and returns the result (rounded down) in a new `ProficiencyBonus`.
    pub const fn divide_rounded_down(&self, n: u8) -> Self {
        let Self(value) = self;
        Self(*value / n)
    }

    /// Divides this object by `n` and returns the result (rounded up) in a new `ProficiencyBonus`.
    pub const fn divide_rounded_up(&self, n: u8) -> Self {
        let Self(value) = self;
        Self((*value + (n - 1)) / n)
    }
}

impl From<u8> for ProficiencyBonus {
    fn from(item: u8) -> Self {
        Self(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bonus_multiply() {
        let bonus = ProficiencyBonus(3);
        assert_eq!(bonus.multiply(2), 6.into());
    }

    #[test]
    fn bonus_divide() {
        let bonus = ProficiencyBonus(3);
        assert_eq!(bonus.divide_rounded_down(2), 1.into());
        assert_eq!(bonus.divide_rounded_up(2), 2.into());
    }
}
