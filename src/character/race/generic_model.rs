//! Generic race model.

use crate::ability::{AbilityId, AbilityScore};
use crate::character::race::RaceModel;

/// A generic race model implementation.
#[derive(Default, new)]
pub struct GenericRaceModel {
    ability_score_increase: Vec<(AbilityId, AbilityScore)>,
}

impl GenericRaceModel {
    /// Adds a new ability score increase.
    pub fn add_ability_score_increase(&mut self, id: AbilityId, score: AbilityScore) -> &mut Self {
        self.ability_score_increase.push((id, score));
        self
    }
}

impl RaceModel for GenericRaceModel {
    fn ability_score_increase(&self) -> &[(AbilityId, AbilityScore)] {
        &self.ability_score_increase
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let model = GenericRaceModel::new(vec![
            (AbilityId(0), AbilityScore::capped(1)),
            (AbilityId(2), AbilityScore::capped(2)),
        ]);
        assert_eq!(model.ability_score_increase().len(), 2);
    }
}
