//! Fighter class.

use crate::character::class::ClassModel;

/// Id of the Fighter class.
///
/// # Reference
///
/// **Hit Dice:** 1d10 per fighter level\
/// **Hit Points at 1st Level: 10 + your Constitution modifier
pub const FIGHTER: &str = "_fighter";

#[derive(Default)]
/// Models the Fighter class.
pub(crate) struct FighterModel {}

impl ClassModel for FighterModel {}
