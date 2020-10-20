//! Generic race model.

use crate::ability::{AbilityId, AbilityScore};
use crate::character::race::RaceModel;
use crate::rules::core::size::CreatureSize;

/// A generic race model implementation.
#[derive(new)]
pub struct GenericRaceModel {
    #[new(default)]
    ability_score_increases: Vec<(AbilityId, AbilityScore)>,
    size: CreatureSize,
}

impl GenericRaceModel {
    /// Adds a new ability score increase.
    pub fn add_ability_score_increase(&mut self, id: AbilityId, score: AbilityScore) -> &mut Self {
        self.ability_score_increases.push((id, score));
        self
    }
}

impl RaceModel for GenericRaceModel {
    fn ability_score_increases(&self) -> &[(AbilityId, AbilityScore)] {
        &self.ability_score_increases
    }

    fn size(&self) -> CreatureSize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let mut model = GenericRaceModel::new(CreatureSize::Medium);
        model.add_ability_score_increase(AbilityId(0), AbilityScore::capped(1));
        model.add_ability_score_increase(AbilityId(2), AbilityScore::capped(2));
        assert_eq!(model.ability_score_increases().len(), 2);
        assert_eq!(model.size(), CreatureSize::Medium);
    }
}
