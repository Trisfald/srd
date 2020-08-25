//! Implementation of rules for actors.

use crate::rules::core::action::{Action, ActionsSeed};
use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{Actor, ActorRules, BattleState, Entropy, EventQueue, WeaselResult, WriteMetrics};

/// Rules to manage abilities that can be activated and any action a character can take.
pub struct SRDActorRules {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<dyn Narrator>,
}

impl SRDActorRules {
    /// Creates a new instance.
    pub(crate) fn new(narrator: Arc<dyn Narrator>) -> Self {
        Self { narrator }
    }
}

impl ActorRules<SRDRules> for SRDActorRules {
    type Ability = Action;
    type AbilitiesSeed = ActionsSeed;
    type Activation = (); // TODO use a real type
    type AbilitiesAlteration = (); // TODO use a real type

    fn generate_abilities(
        &self,
        seed: &Option<Self::AbilitiesSeed>,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) -> Box<dyn Iterator<Item = Self::Ability>> {
        if let Some(seed) = seed {
            // Generate an ability out of each single ActionInitializer.
            log::trace!("generating {} abilities", seed.actions.len());
            Box::new(seed.actions.clone().into_iter().map(|e| e.into()))
        } else {
            log::warn!("generating an empty set of abilities for a weasel::actor");
            Box::new(std::iter::empty())
        }
    }

    fn activable(
        &self,
        _state: &BattleState<SRDRules>,
        _action: weasel::Action<SRDRules>,
    ) -> WeaselResult<(), SRDRules> {
        unimplemented!()
    }

    fn activate(
        &self,
        _state: &BattleState<SRDRules>,
        _action: weasel::Action<SRDRules>,
        _event_queue: &mut Option<EventQueue<SRDRules>>,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) {
        unimplemented!()
    }

    fn alter_abilities(
        &self,
        _actor: &mut dyn Actor<SRDRules>,
        _alteration: &Self::AbilitiesAlteration,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) {
        unimplemented!()
    }
}
