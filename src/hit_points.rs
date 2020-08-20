//! Hit Points of creatures.

use crate::dice::Dice;
use crate::value::PositiveBoundedValue;

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
