//! Size and dimension of entities.

use crate::rules::core::constants::SQUARE_FT;
use serde::{Deserialize, Serialize};

/// Each creature takes up a different amount of space.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CreatureSize {
    /// 2½ by 2½ ft.
    Tiny,
    /// 5 by 5 ft.
    Small,
    /// 5 by 5 ft.
    Medium,
    /// 10 by 10 ft.
    Large,
    /// 15 by 15 ft.
    Huge,
    /// 20 by 20 ft.
    Gargantuan,
}

impl CreatureSize {
    /// Returns the length (in squares) of one side of the occupied space.
    ///
    /// # Reference
    ///
    /// A creature's space is the area that it effectively controls in combat, not an expression of its physical dimensions.
    pub const fn space_sq(&self) -> u8 {
        use CreatureSize::*;
        match self {
            Tiny => 1,
            Small => 2,
            Medium => 2,
            Large => 4,
            Huge => 6,
            Gargantuan => 8,
        }
    }

    /// Returns the length (in feet) of one side of the occupied space.\
    /// See also [space](enum.CreatureSize.html#method.space).
    pub fn space_ft(&self) -> f32 {
        f32::from(self.space_sq()) * SQUARE_FT
    }
}
