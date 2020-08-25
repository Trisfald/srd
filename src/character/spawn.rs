//! Spawn system.

use crate::character::Character;
use crate::compendium::compendium;
use crate::error::{SRDError, SRDResult};
use crate::rules::core::action::ActionInitializer;
use crate::rules::core::action::ActionsSeed;
use crate::rules::core::statistic::StatisticInitializer;
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
        log::debug!("spawning character {:?}", self.character.id());
        // Get the creature's statistics and actions.
        let statistics = self.build_statistics()?;
        let actions = self.build_actions();
        // We need a team in order to spawn a creature.
        create_global_team(server)?;
        // Now we can finally spawn the character's creature.
        CreateCreature::trigger(server, self.character.id().clone(), GLOBAL_TEAM_ID, ())
            .statistics_seed(statistics)
            .abilities_seed(actions)
            .fire()?;
        log::info!("spawned character {:?}", self.character.id());
        Ok(())
    }

    fn build_statistics(&self) -> SRDResult<StatisticsSeed> {
        let mut seed = StatisticsSeed::default();
        self.add_base_statistics(&mut seed)?;
        self.add_abilities(&mut seed);
        self.add_skills(&mut seed);
        Ok(seed)
    }

    fn add_base_statistics(&self, seed: &mut StatisticsSeed) -> SRDResult<()> {
        use StatisticInitializer::*;
        seed.statistics.push(Race(self.character.race().clone()));
        seed.statistics.push(Class(self.character.class().clone()));
        seed.statistics.push(Level(self.character.level().clone()));
        let class = self.character.class();
        let class_model = compendium()
            .class_model(class)
            .ok_or(SRDError::ClassNotFound(class.clone()))?;
        seed.statistics.push(ProficiencyBonus(
            class_model.proficiency_bonus(self.character.level()),
        ));
        seed.statistics
            .push(HitPoints(self.character.hit_points().clone()));
        Ok(())
    }

    fn add_abilities(&self, seed: &mut StatisticsSeed) {
        for (id, score) in self.character.abilities() {
            seed.statistics
                .push(StatisticInitializer::Ability(*id, *score));
        }
    }

    fn add_skills(&self, seed: &mut StatisticsSeed) {
        for (id, proficiency) in self.character.skills() {
            seed.statistics
                .push(StatisticInitializer::Skill(*id, *proficiency));
        }
    }

    fn build_actions(&self) -> ActionsSeed {
        let mut seed = ActionsSeed::default();
        self.add_base_actions(&mut seed);
        seed
    }

    fn add_base_actions(&self, seed: &mut ActionsSeed) {
        use ActionInitializer::*;
        seed.actions.push(Movement);
        seed.actions.push(Attack);
    }
}

#[cfg(test)]
mod tests {
    use crate::ability::RESERVED_ABILITIES;
    use crate::ability::{AbilityScore, DEFAULT_ABILITY_SCORE, DEXTERITY, STRENGTH};
    use crate::character::class::fighter::FIGHTER;
    use crate::character::race::hill_dwarf::HILL_DWARF;
    use crate::character::CharacterId;
    use crate::proficiency::{Proficiency, DEFAULT_PROFICIENCY};
    use crate::rules::core::action::ActionId;
    use crate::rules::core::statistic::StatisticId;
    use crate::skill::{ACROBATICS, RESERVED_SKILLS, STEALTH};
    use crate::util::simple_server;
    use weasel::{Actor, BattleController, Character};

    #[test]
    fn character_has_correct_statistics() {
        let strength = 12;
        let id: CharacterId = "one".into();
        let mut server = simple_server();

        // Spawn a character with one non default ability and skill.
        crate::Character::new(id.clone(), HILL_DWARF, FIGHTER)
            .unwrap()
            .add_ability(STRENGTH, AbilityScore::new(strength).unwrap())
            .add_skill(ACROBATICS, Proficiency(true))
            .spawn(&mut server)
            .unwrap();
        // The creature should have the ability we set and everything else defaulted.
        let creature = server.battle().entities().creature(&id).unwrap();
        let expected_statistics: usize = (RESERVED_ABILITIES + RESERVED_SKILLS + 5).into();
        assert_eq!(creature.statistics().count(), expected_statistics);
        assert_eq!(
            *creature
                .statistic(&StatisticId::Ability(STRENGTH))
                .unwrap()
                .ability()
                .unwrap(),
            AbilityScore::capped(strength)
        );
        assert_eq!(
            *creature
                .statistic(&StatisticId::Ability(DEXTERITY))
                .unwrap()
                .ability()
                .unwrap(),
            DEFAULT_ABILITY_SCORE
        );
        // The creature should have the skill we set and everything else defaulted.
        assert_eq!(
            *creature
                .statistic(&StatisticId::Skill(ACROBATICS))
                .unwrap()
                .skill()
                .unwrap(),
            Proficiency(true)
        );
        assert_eq!(
            *creature
                .statistic(&StatisticId::Skill(STEALTH))
                .unwrap()
                .skill()
                .unwrap(),
            DEFAULT_PROFICIENCY
        );
    }

    #[test]
    fn character_has_default_actions() {
        let id: CharacterId = "one".into();
        let mut server = simple_server();

        // Spawn a character.
        crate::Character::new(id.clone(), HILL_DWARF, FIGHTER)
            .unwrap()
            .spawn(&mut server)
            .unwrap();
        // The creature should have all default actions.
        let creature = server.battle().entities().creature(&id).unwrap();
        assert!(creature.ability(&ActionId::Movement).is_some());
        assert!(creature.ability(&ActionId::Attack).is_some());
    }
}
