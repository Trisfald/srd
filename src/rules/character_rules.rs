//! Implementation of rules for characters.

use crate::character::CharacterId;
use crate::rules::core::statistic::{Statistic, StatisticChange, StatisticsSeed};
use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{Character, CharacterRules, Entropy, Transmutation, WriteMetrics};

/// Rules for representing and evolving characters.\
/// Character in weasel has a broader definition since it includes objects as well.
pub struct SRDCharacterRules {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<dyn Narrator>,
}

impl SRDCharacterRules {
    /// Creates a new instance.
    pub(crate) fn new(narrator: Arc<dyn Narrator>) -> Self {
        Self { narrator }
    }
}

impl CharacterRules<SRDRules> for SRDCharacterRules {
    type CreatureId = CharacterId;
    type ObjectId = (); // TODO use a real type
    type Statistic = Statistic;
    type StatisticsSeed = StatisticsSeed;
    type StatisticsAlteration = StatisticChange;
    type Status = weasel::rules::empty::EmptyStatus; // TODO not sure if we need this
    type StatusesAlteration = (); // TODO not sure if we need this

    fn generate_statistics(
        &self,
        seed: &Option<Self::StatisticsSeed>,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) -> Box<dyn Iterator<Item = Self::Statistic>> {
        if let Some(seed) = seed {
            // Generate a statistic out of each single StatisticInitializer.
            Box::new(seed.statistics.clone().into_iter().map(|e| e.into()))
        } else {
            log::warn!("generating an empty set of statistics for a weasel::character");
            Box::new(std::iter::empty())
        }
    }

    fn alter_statistics(
        &self,
        _character: &mut dyn Character<SRDRules>,
        _alteration: &Self::StatisticsAlteration,
        _entropy: &mut Entropy<SRDRules>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) -> Option<Transmutation> {
        unimplemented!()
    }
}
