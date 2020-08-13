//! Hill Dwarf race.

use crate::character::race::generic_model::GenericRaceModel;

/// Id of the Hill Dwarf race.
///
/// # Reference
///
/// ## Dwarf
/// **Ability Score Increase.** Your Constitution score increases by 2.
///
/// ## Hill Dwarf
/// **Ability Score Increase.** Your Wisdom score increases by 1.
pub const HILL_DWARF: &str = "_hill_dwarf";

/// Returns the race model for Hill Dwarf.
pub fn hill_dwarf_model() -> GenericRaceModel {
    GenericRaceModel::default()
}
