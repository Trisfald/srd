//! Fighter class.

use crate::character::class::ClassModel;
use crate::dice::Die;
use crate::hit_points::HitDice;

/// Id of the Fighter class.
///
/// # Reference
///
/// **Hit Dice:** 1d10 per fighter level\
/// **Hit Points at 1st Level: 10 + your Constitution modifier
/// **Hit Points at Higher Levels: 1d10 (or 6) + your Constitution modifier per fighter level after 1s
///
/// |Level|Proficiency Bonus|Features|
/// |:---:|:---------------:|--------|
/// |1st|+2|Fighting Style, Second Wind
pub const FIGHTER: &str = "_fighter";

#[derive(Default)]
/// Models the Fighter class.
pub(crate) struct FighterModel {}

impl ClassModel for FighterModel {
    fn hit_dice(&self) -> HitDice {
        HitDice::new(1, Die::D10)
    }
}
