//! Spawn system.

use crate::character::Character;
use crate::error::SRDResult;
use crate::rules::core::action::ActionsSeed;
use crate::rules::core::statistic::StatisticsSeed;
use crate::rules::team_rules::{create_global_team, GLOBAL_TEAM_ID};
use crate::rules::SRDRules;
use weasel::{CreateCreature, EventTrigger, Server};

/// Helper to spawn a creature for the given character.
#[derive(new)]
pub(crate) struct CharacterSpawner<'a> {
    character: &'a Character,
}

impl CharacterSpawner<'_> {
    pub(crate) fn spawn(&self, server: &mut Server<SRDRules>) -> SRDResult<()> {
        log::info!("spawning character {:?}", self.character.id());
        let statistics = self.build_statistics();
        let actions = self.build_actions();
        // We need a team in order to spawn a creature.
        create_global_team(server)?;
        // Now we can finally spawn the character's creature.
        CreateCreature::trigger(server, self.character.id().clone(), GLOBAL_TEAM_ID, ())
            .statistics_seed(statistics)
            .abilities_seed(actions)
            .fire()?;
        Ok(())
    }

    fn build_statistics(&self) -> StatisticsSeed {
        StatisticsSeed::default()
        // TODO
    }

    fn build_actions(&self) -> ActionsSeed {
        ActionsSeed::default()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_has_correct_statistics() {
        // Spawn a character with one ability.
        // The creature should have the ability we set and everything else defaulted.
    }

    #[test]
    fn character_has_default_actions() {
        // Spawn a character.
        // The creature should have all default actions.
    }
}
