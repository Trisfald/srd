//! Implementation of rules for the order of initiative.

use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{Actor, Entities, Entropy, RoundsRules, Space, WriteMetrics};

/// Rules to determine the order of initiative during a battle.
pub struct SRDRoundsRules {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<dyn Narrator>,
}

impl SRDRoundsRules {
    /// Creates a new instance.
    pub(crate) fn new(narrator: Arc<dyn Narrator>) -> Self {
        Self { narrator }
    }
}

impl RoundsRules<SRDRules> for SRDRoundsRules {
    type RoundsSeed = (); // TODO use a real type
    type RoundsModel = (); // TODO use a real type

    fn generate_model(&self, _: &Option<Self::RoundsSeed>) -> Self::RoundsModel {
        ()
    }

    fn eligible(&self, _model: &Self::RoundsModel, _actor: &dyn Actor<SRDRules>) -> bool {
        unimplemented!()
    }

    fn on_start(
        &self,
        _entities: &Entities<SRDRules>,
        _space: &Space<SRDRules>,
        _model: &mut Self::RoundsModel,
        _actor: &dyn Actor<SRDRules>,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) {
        unimplemented!()
    }

    fn on_actor_added(
        &self,
        _model: &mut Self::RoundsModel,
        _actor: &dyn Actor<SRDRules>,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) {
        // TODO
    }

    fn on_actor_removed(
        &self,
        _model: &mut Self::RoundsModel,
        _actor: &dyn Actor<SRDRules>,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) {
        unimplemented!()
    }
}
