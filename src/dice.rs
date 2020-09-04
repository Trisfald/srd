//! Die and dice module.

use serde::{Deserialize, Serialize};

const DIE_TYPES_COUNT: usize = 7;

/// Types of die used in the SRD.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Die {
    /// A die with four sides.
    D4,
    /// A die with six sides.
    D6,
    /// A die with eight sides.
    D8,
    /// A die with ten sides.
    D10,
    /// A die with twelve sides.
    D12,
    /// A die with twenty sides.
    D20,
    /// A die with one hunded sides.
    D100,
}

impl Die {
    /// Returns the number of sides of a die.
    pub const fn sides(&self) -> u8 {
        use Die::*;
        match self {
            D4 => 4,
            D6 => 6,
            D8 => 8,
            D10 => 10,
            D12 => 12,
            D20 => 20,
            D100 => 100,
        }
    }

    // Id of a die.
    const fn id(&self) -> usize {
        use Die::*;
        match self {
            D4 => 0,
            D6 => 1,
            D8 => 2,
            D10 => 3,
            D12 => 4,
            D20 => 5,
            D100 => 6,
        }
    }

    /// The minimum obtainable value from this die.
    pub const fn min(&self) -> u8 {
        1
    }

    /// The maximum obtainable value from this die.
    pub const fn max(&self) -> u8 {
        self.sides()
    }
}

/// A dice represents how many times a particular die should be rolled.
/// It is the primary way to resolve random actions in the SRD.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, new)]
pub struct Dice {
    /// Number of times the die should be rolled.
    pub n: u8,
    /// The type of die to use.
    pub die: Die,
}

/// A list of dice. Dice(s) are compacted: an individual die is repeated only once.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DicePool {
    /// An array with the number of times each die should be rolled.
    pub pool: [u8; DIE_TYPES_COUNT],
}

impl DicePool {
    /// Constructs a new `DicePool` from a simple dice.
    pub fn from_dice(dice: Dice) -> Self {
        let mut instance = Self::default();
        instance.add_dice(dice);
        instance
    }

    /// Adds a new dice.
    pub fn add_dice(&mut self, dice: Dice) -> &mut Self {
        self.pool[dice.die.id()] += dice.n;
        self
    }
}

/// A dice roll is a `Dice` plus a modifier to be added to the final result.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DiceRoll {
    /// The dice to be rolled.
    pub dice: Dice,
    /// A numerical modifier to be added to the dice roll's result.
    pub modifier: i16,
}

impl DiceRoll {
    /// Constructs a new `DiceRoll`.
    pub const fn new(dice: Dice) -> Self {
        Self { dice, modifier: 0 }
    }

    /// Constructs a new `DiceRoll` with an additional modifier.
    pub const fn with_modifier(dice: Dice, modifier: i16) -> Self {
        Self { dice, modifier }
    }
}

/// Similar to `DicePool` with the addition of a modifier.
/// Internal dice roll(s) are compacted, in the sense that
/// there's a single modifier and roll(s) of an individual die are repeated only once.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DiceRolls {
    /// The dice pool to be rolled.
    pub dice_pool: DicePool,
    /// A numerical modifier to be added to the dice rolls' result.
    pub modifier: i16,
}

impl DiceRolls {
    /// Constructs a new `DiceRolls` starting from a single `DiceRoll`.
    pub fn from_roll(roll: DiceRoll) -> Self {
        let mut instance = Self {
            dice_pool: DicePool::default(),
            modifier: 0,
        };
        instance.add_roll(roll);
        instance
    }

    /// Constructs a new `DiceRolls` starting from a `DicePool`.
    pub fn from_pool(dice: DicePool) -> Self {
        Self {
            dice_pool: dice,
            modifier: 0,
        }
    }

    /// Adds a new dice.
    pub fn add_dice(&mut self, dice: Dice) -> &mut Self {
        self.dice_pool.add_dice(dice);
        self
    }

    /// Adds a new `DiceRoll`.
    pub fn add_roll(&mut self, roll: DiceRoll) -> &mut Self {
        self.add_dice(roll.dice);
        self.modifier += roll.modifier;
        self
    }
}

impl From<DiceRoll> for DiceRolls {
    fn from(item: DiceRoll) -> Self {
        Self::from_roll(item)
    }
}

impl From<DicePool> for DiceRolls {
    fn from(item: DicePool) -> Self {
        Self::from_pool(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_pool() {
        let mut dc = DicePool::default();
        assert_eq!(dc.pool, [0, 0, 0, 0, 0, 0, 0]);
        dc.add_dice(Dice::new(1, Die::D4))
            .add_dice(Dice::new(1, Die::D8));
        assert_eq!(dc.pool, [1, 0, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn dice_rolls_add() {
        let mut dc = DiceRolls::from_roll(DiceRoll::with_modifier(Dice::new(2, Die::D4), 3));
        assert_eq!(dc.dice_pool.pool, [2, 0, 0, 0, 0, 0, 0]);
        assert_eq!(dc.modifier, 3);
        dc.add_roll(DiceRoll::new(Dice::new(5, Die::D20)));
        assert_eq!(dc.dice_pool.pool, [2, 0, 0, 0, 0, 5, 0]);
        assert_eq!(dc.modifier, 3);
        dc.add_roll(DiceRoll::with_modifier(Dice::new(1, Die::D20), -4));
        assert_eq!(dc.dice_pool.pool, [2, 0, 0, 0, 0, 6, 0]);
        assert_eq!(dc.modifier, -1);
        dc.add_dice(Dice::new(1, Die::D6));
        assert_eq!(dc.dice_pool.pool, [2, 1, 0, 0, 0, 6, 0]);
        assert_eq!(dc.modifier, -1);
    }

    #[test]
    fn dice_rolls_from_pool() {
        let dc = DiceRolls::from_pool(DicePool::from_dice(Dice::new(2, Die::D4)));
        assert_eq!(dc.dice_pool.pool, [2, 0, 0, 0, 0, 0, 0]);
        assert_eq!(dc.modifier, 0);
    }
}
