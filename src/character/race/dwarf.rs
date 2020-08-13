//! Dwarf race.

use crate::character::race::generic_model::GenericRaceModel;
use crate::ability::{AbilityScore, CONSTITUTION};

/// Returns the race model for Dwarf.
pub fn dwarf_model() -> GenericRaceModel {
    let mut model = GenericRaceModel::default();
    model.add_ability_score_increase(CONSTITUTION, AbilityScore::capped(2));
    model
}
