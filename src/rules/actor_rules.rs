//! Implementation of rules for actors.

use crate::rules::core::action::{Action, ActionsSeed};
use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{Actor, ActorRules, BattleState, Entropy, EventQueue, WeaselResult, WriteMetrics};

/// Rules to manage abilities that can be activated and any action a character can take.
pub struct SRDActorRules<N: Narrator> {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<N>,
}

impl<'a, N: Narrator> SRDActorRules<N> {
    /// Creates a new instance.
    pub fn new(narrator: Arc<N>) -> Self {
        Self { narrator }
    }
}

impl<N: Narrator> ActorRules<SRDRules<N>> for SRDActorRules<N> {
    type Ability = Action;
    type AbilitiesSeed = ActionsSeed;
    type Activation = (); // TODO use a real type
    type AbilitiesAlteration = (); // TODO use a real type

    fn generate_abilities(
        &self,
        _seed: &Option<Self::AbilitiesSeed>,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) -> Box<dyn Iterator<Item = Self::Ability>> {
        unimplemented!()
    }

    fn activable(
        &self,
        _state: &BattleState<SRDRules<N>>,
        _action: weasel::Action<SRDRules<N>>,
    ) -> WeaselResult<(), SRDRules<N>> {
        unimplemented!()
    }

    fn activate(
        &self,
        _state: &BattleState<SRDRules<N>>,
        _action: weasel::Action<SRDRules<N>>,
        _event_queue: &mut Option<EventQueue<SRDRules<N>>>,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) {
        unimplemented!()
    }

    fn alter_abilities(
        &self,
        _actor: &mut dyn Actor<SRDRules<N>>,
        _alteration: &Self::AbilitiesAlteration,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) {
        unimplemented!()
    }
}
