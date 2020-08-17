//! Implementation of rules for teams.

use crate::character::CharacterId;
use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::TeamRules;

/// Teams do not exists per se in the SRD, but weasel requires them.\
/// The easy solution is have a one to one mapping between character and team.
/// There's no need to implement any specific behavior for teams.
pub struct SRDTeamRules<N: Narrator> {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<N>,
}

impl<N: Narrator> SRDTeamRules<N> {
    /// Creates a new instance.
    pub fn new(narrator: Arc<N>) -> Self {
        Self { narrator }
    }
}

impl<N: Narrator> TeamRules<SRDRules<N>> for SRDTeamRules<N> {
    // Same as the Id for Characters.
    type Id = CharacterId;
    type ObjectivesSeed = ();
    type Objectives = ();
}
