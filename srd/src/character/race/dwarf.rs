//! Dwarf race.

use crate::ability::{AbilityScore, CONSTITUTION};
use crate::character::race::generic_model::GenericRaceModel;

/// Returns the race model for Dwarf.
pub fn dwarf_model() -> GenericRaceModel {
    let mut model = GenericRaceModel::default();
    model.add_ability_score_increase(CONSTITUTION, AbilityScore::capped(2));
    model
}
