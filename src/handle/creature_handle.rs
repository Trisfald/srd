//! Handle for creatures.

use crate::ability::{AbilityId, AbilityScore};
use crate::character::class::ClassId;
use crate::character::level::Level;
use crate::character::race::RaceId;
use crate::error::{SRDError, SRDResult};
use crate::proficiency::Proficiency;
use crate::rules::core::action::Action;
use crate::rules::core::StatisticId;
use crate::rules::SRDRules;
use crate::skill::SkillId;
use weasel::{
    creature::CreatureId, space::Position, Actor, BattleController, Character, Creature, Entity,
    EventProcessor, Id, WeaselError, WeaselResult,
};

/// This macro generates all methods for a `CreatureHandle(Mut)` that do not require a mutable access
/// to the event processor.
macro_rules! generate_immutable_methods {
    () => {
        /// Returns a reference to the `weasel::Creature`.
        fn creature(&self) -> WeaselResult<&Creature<SRDRules>, SRDRules> {
            self.controller
                .battle()
                .entities()
                .creature(&self.id)
                .ok_or_else(|| WeaselError::CreatureNotFound(self.id.clone()))
        }

        /// Returns the `CreatureId`.
        pub fn id(&self) -> &CreatureId<SRDRules> {
            &self.id
        }

        /// Returns the creature's race.
        ///
        /// # Errors
        ///
        /// An error is returned if the creature no longer exists or if it doesn't have a race.
        ///
        pub fn race(&self) -> SRDResult<&RaceId> {
            self.creature()?
                .statistic(&StatisticId::Race)
                .ok_or_else(|| SRDError::StatisticNotFound(StatisticId::Race))?
                .race()
        }

        /// Returns the creature's class.
        ///
        /// # Errors
        ///
        /// An error is returned if the creature no longer exists or if it doesn't have a class.
        ///
        pub fn class(&self) -> SRDResult<&ClassId> {
            self.creature()?
                .statistic(&StatisticId::Class)
                .ok_or_else(|| SRDError::StatisticNotFound(StatisticId::Class))?
                .class()
        }

        /// Returns the creature's level.
        ///
        /// # Errors
        ///
        /// An error is returned if the creature no longer exists or if it doesn't have a level.
        ///
        pub fn level(&self) -> SRDResult<&Level> {
            self.creature()?
                .statistic(&StatisticId::Level)
                .ok_or_else(|| SRDError::StatisticNotFound(StatisticId::Level))?
                .level()
        }

        /// Returns the creature's position.
        ///
        /// # Errors
        ///
        /// An error is returned if the creature no longer exists.
        ///
        pub fn position(&self) -> SRDResult<&Position<SRDRules>> {
            Ok(self.creature()?.position())
        }

        /// Returns an iterator over the creature's abilities.
        ///
        /// # Errors
        ///
        /// An error is returned if the creature no longer exists.
        ///
        pub fn abilities(&self) -> SRDResult<impl Iterator<Item = (&AbilityId, &AbilityScore)>> {
            Ok(self.creature()?.statistics().filter_map(|e| {
                if let StatisticId::Ability(id) = e.id() {
                    e.ability().map_or(None, |score| Some((id, score)))
                } else {
                    None
                }
            }))
        }

        /// Returns an iterator over the creature's skill proficiencies.
        ///
        /// # Errors
        ///
        /// An error is returned if the creature no longer exists.
        ///
        pub fn skills(&self) -> SRDResult<impl Iterator<Item = (&SkillId, &Proficiency)>> {
            Ok(self.creature()?.statistics().filter_map(|e| {
                if let StatisticId::Skill(id) = e.id() {
                    e.skill().map_or(None, |prof| Some((id, prof)))
                } else {
                    None
                }
            }))
        }

        /// Returns an iterator over the creature's actions.
        ///
        /// # Errors
        ///
        /// An error is returned if the creature no longer exists.
        ///
        pub fn actions(&self) -> SRDResult<impl Iterator<Item = &Action>> {
            Ok(self.creature()?.abilities())
        }
    };
}

/// A `CreatureHandle` can be used to read the state of an entity and what actions it
/// can take.
#[derive(new)]
pub struct CreatureHandle<'a, C: BattleController<SRDRules>> {
    id: &'a CreatureId<SRDRules>,
    controller: &'a C,
}

impl<'a, C: BattleController<SRDRules>> CreatureHandle<'a, C> {
    generate_immutable_methods! {}
}

/// A `CreatureHandleMut` can be used to read the state of an entity, what actions it
/// can take and to issue commands.
#[derive(new)]
pub struct CreatureHandleMut<'a, C>
where
    C: BattleController<SRDRules> + EventProcessor<SRDRules>,
{
    id: &'a CreatureId<SRDRules>,
    controller: &'a mut C,
}

impl<'a, C> CreatureHandleMut<'a, C>
where
    C: BattleController<SRDRules> + EventProcessor<SRDRules>,
{
    generate_immutable_methods! {}
}

impl<'a, C> From<CreatureHandleMut<'a, C>> for CreatureHandle<'a, C>
where
    C: BattleController<SRDRules> + EventProcessor<SRDRules>,
{
    fn from(item: CreatureHandleMut<'a, C>) -> Self {
        CreatureHandle::new(item.id, item.controller)
    }
}
