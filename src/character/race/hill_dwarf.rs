//! Hill Dwarf race.

use crate::character::race::{RaceId, RaceModel};

/// Id of the Hill Dwarf race.
///
/// # Reference
///
/// ## Dwarf
/// **Ability Score Increase.** Your Constitution score increases by 2.
///
/// ## Hill Dwarf
/// **Ability Score Increase.** Your Wisdom score increases by 1.
pub const HILL_DWARF: RaceId = RaceId(0);

#[derive(Default)]
/// Models the Hill Dwarf race.
pub(crate) struct HillDwarfModel {}

impl RaceModel for HillDwarfModel {}
