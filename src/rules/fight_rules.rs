//! Implementation of rules for combat.

use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{BattleState, Entropy, EventQueue, FightRules, WriteMetrics};

/// Rules to manage combat and damage.
pub struct SRDFightRules {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<dyn Narrator>,
}

impl SRDFightRules {
    /// Creates a new instance.
    pub(crate) fn new(narrator: Arc<dyn Narrator>) -> Self {
        Self { narrator }
    }
}

impl FightRules<SRDRules> for SRDFightRules {
    type Impact = (); // TODO use a real type
    type Potency = (); // TODO not sure if we need this

    fn apply_impact(
        &self,
        _state: &BattleState<SRDRules>,
        _impact: &Self::Impact,
        _event_queue: &mut Option<EventQueue<SRDRules>>,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) {
        unimplemented!()
    }
}
