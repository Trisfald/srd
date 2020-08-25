//! Implementation of rules for combat.

use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{BattleState, Entropy, EventQueue, FightRules, WriteMetrics};

/// Rules to manage combat and damage.
pub struct SRDFightRules<N: Narrator> {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<N>,
}

impl<N: Narrator> SRDFightRules<N> {
    /// Creates a new instance.
    pub fn new(narrator: Arc<N>) -> Self {
        Self { narrator }
    }
}

impl<N: Narrator> FightRules<SRDRules<N>> for SRDFightRules<N> {
    type Impact = (); // TODO use a real type
    type Potency = (); // TODO not sure if we need this

    fn apply_impact(
        &self,
        _state: &BattleState<SRDRules<N>>,
        _impact: &Self::Impact,
        _event_queue: &mut Option<EventQueue<SRDRules<N>>>,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) {
        unimplemented!()
    }
}
