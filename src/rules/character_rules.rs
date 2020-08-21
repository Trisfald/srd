//! Implementation of rules for characters.

use crate::character::CharacterId;
use crate::rules::core::statistic::{Statistic, StatisticChange, StatisticsSeed};
use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{Character, CharacterRules, Entropy, Transmutation, WriteMetrics};

/// Rules for representing and evolving characters.\
/// Character in weasel has a broader definition since it includes objects as well.
#[derive(Default)]
pub struct SRDCharacterRules<N: Narrator> {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<N>,
}

impl<N: Narrator> SRDCharacterRules<N> {
    /// Creates a new instance.
    pub fn new(narrator: Arc<N>) -> Self {
        Self { narrator }
    }
}

impl<N: Narrator> CharacterRules<SRDRules<N>> for SRDCharacterRules<N> {
    type CreatureId = CharacterId;
    type ObjectId = (); // TODO use a real type
    type Statistic = Statistic;
    type StatisticsSeed = StatisticsSeed;
    type StatisticsAlteration = StatisticChange;
    type Status = weasel::rules::empty::EmptyStatus; // TODO not sure if we need this
    type StatusesAlteration = (); // TODO not sure if we need this

    fn generate_statistics(
        &self,
        _seed: &Option<Self::StatisticsSeed>,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) -> Box<dyn Iterator<Item = Self::Statistic>> {
        unimplemented!()
    }

    fn alter_statistics(
        &self,
        _character: &mut dyn Character<SRDRules<N>>,
        _alteration: &Self::StatisticsAlteration,
        _entropy: &mut Entropy<SRDRules<N>>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) -> Option<Transmutation> {
        unimplemented!()
    }
}
