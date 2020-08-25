//! Implementation of rules for the order of initiative.

use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{Actor, Entities, Entropy, RoundsRules, Space, WriteMetrics};

/// Rules to determine the order of initiative during a battle.
pub struct SRDRoundsRules<N: Narrator> {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<N>,
}

impl<N: Narrator> SRDRoundsRules<N> {
    /// Creates a new instance.
    pub fn new(narrator: Arc<N>) -> Self {
        Self { narrator }
    }
}

impl<N: Narrator> RoundsRules<SRDRules<N>> for SRDRoundsRules<N> {
    type RoundsSeed = (); // TODO use a real type
    type RoundsModel = (); // TODO use a real type

    fn generate_model(&self, _: &Option<Self::RoundsSeed>) -> Self::RoundsModel {
        unimplemented!()
    }

    fn eligible(&self, _model: &Self::RoundsModel, _actor: &dyn Actor<SRDRules<N>>) -> bool {
        unimplemented!()
    }

    fn on_start(
        &self,
        _entities: &Entities<SRDRules<N>>,
        _space: &Space<SRDRules<N>>,
        _model: &mut Self::RoundsModel,
        _actor: &dyn Actor<SRDRules<N>>,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) {
        unimplemented!()
    }

    fn on_actor_added(
        &self,
        _model: &mut Self::RoundsModel,
        _actor: &dyn Actor<SRDRules<N>>,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) {
        unimplemented!()
    }

    fn on_actor_removed(
        &self,
        _model: &mut Self::RoundsModel,
        _actor: &dyn Actor<SRDRules<N>>,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) {
        unimplemented!()
    }
}
