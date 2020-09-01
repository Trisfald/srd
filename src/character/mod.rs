//! Playable and non-playable characters.

pub mod class;
pub use self::class::{ClassId, ClassModel};

pub mod level;
pub use self::level::Level;

pub mod race;
pub use self::race::{RaceId, RaceModel};

mod spawn;

use self::spawn::CharacterSpawner;
use crate::ability::{AbilityId, AbilityScore, CONSTITUTION, DEFAULT_ABILITY_SCORE};
use crate::compendium::compendium;
use crate::error::{SRDError, SRDResult};
use crate::handle::creature_handle::CreatureHandleMut;
use crate::hit_points::{HitPoints, HitPointsHistory};
use crate::proficiency::{Proficiency, DEFAULT_PROFICIENCY};
use crate::rules::SRDRules;
use crate::skill::SkillId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use weasel::Server;

/// A character unique identifier.
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash, Eq, Serialize, Deserialize)]
pub struct CharacterId(pub String);

impl From<&str> for CharacterId {
    fn from(item: &str) -> Self {
        CharacterId(item.to_string())
    }
}

/// A fictional character. The main entity of a game.
///
/// Objects of this kind contains all information needed to represent a PC (playable character) or
/// a NPC (non-playable character).\
/// This structure does not handle the character in battle. Its purpose, instead, is to be able
/// to spawn an entity having the specified properties. It is necessary because entities lifetime
/// is the battle itself while the same character can partake in multiple battle and even evolve
/// over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    id: CharacterId,
    race: RaceId,
    class: ClassId,
    level: Level,
    hit_points: HitPoints,
    hit_points_history: HitPointsHistory,
    /// The character's abilities.
    abilities: HashMap<AbilityId, AbilityScore>,
    /// The character's proficiency in skills.
    skills: HashMap<SkillId, Proficiency>,
}

impl Character {
    /// Constructs a new `Character`.
    pub fn new<I, R, C>(id: I, race: R, class: C) -> SRDResult<Self>
    where
        I: Into<CharacterId>,
        R: Into<RaceId>,
        C: Into<ClassId>,
    {
        // Verify that the necessary models exist.
        let class = class.into();
        let class_model = compendium()
            .class_model(&class)
            .ok_or_else(|| SRDError::ClassNotFound(class.clone()))?;
        let race = race.into();
        let _ = compendium()
            .race_model(&race)
            .ok_or_else(|| SRDError::RaceNotFound(race.clone()))?;
        // Constructs an instance of this struct.
        let mut hit_points_history = HitPointsHistory::default();
        hit_points_history.add_result(class_model.hit_points_at_1st_level())?;
        let mut instance = Self {
            id: id.into(),
            race,
            class,
            level: Level::default(),
            hit_points: HitPoints::from_value(hit_points_history.total()),
            hit_points_history,
            abilities: HashMap::new(),
            skills: HashMap::new(),
        };
        // Add default abilities and skills.
        Self::add_default_abilities(&mut instance);
        Self::add_default_skills(&mut instance);
        log::debug!("created character {:?}", instance.id);
        Ok(instance)
    }

    fn add_default_abilities(character: &mut Self) {
        log::trace!("adding default abilities to {:?}", character.id);
        for ability in compendium().abilities() {
            character.abilities.insert(*ability, DEFAULT_ABILITY_SCORE);
        }
        character.apply_ability_bonuses();
    }

    fn add_default_skills(character: &mut Self) {
        log::trace!("adding default skills to {:?}", character.id);
        for skill in compendium().skills() {
            character.skills.insert(*skill, DEFAULT_PROFICIENCY);
        }
    }

    /// Returns the character's id.
    pub fn id(&self) -> &CharacterId {
        &self.id
    }

    /// Returns the character's race.
    pub fn race(&self) -> &RaceId {
        &self.race
    }

    /// Returns the character's class.
    pub fn class(&self) -> &ClassId {
        &self.class
    }

    /// Returns the character's level.
    pub fn level(&self) -> &Level {
        &self.level
    }

    /// Returns the character's hit points.
    pub fn hit_points(&self) -> &HitPoints {
        &self.hit_points
    }

    /// Returns an iterator over the character's base ability scores.
    pub fn raw_abilities(&self) -> impl Iterator<Item = (AbilityId, AbilityScore)> + '_ {
        self.abilities.iter().map(|(k, v)| (*k, *v))
    }

    /// Returns an iterator over the character's ability scores (with bonuses).
    pub fn abilities(&self) -> impl Iterator<Item = (AbilityId, AbilityScore)> + '_ {
        let race_model = compendium()
            .race_model(&self.race)
            .expect("race model not found");
        self.raw_abilities().map(move |(ability_id, mut score)| {
            if let Some((_, bonus)) = race_model
                .ability_score_increase()
                .iter()
                .find(|(bonus_id, _)| *bonus_id == ability_id)
            {
                score.add(bonus.value());
            }
            (ability_id, score)
        })
    }

    /// Returns an iterator over the character's skill proficiencies.
    pub fn skills(&self) -> impl Iterator<Item = (SkillId, Proficiency)> + '_ {
        self.skills.iter().map(|(k, v)| (*k, *v))
    }

    /// Adds or replaces one ability.
    pub fn add_ability<A: Into<AbilityId>>(
        &mut self,
        ability: A,
        score: AbilityScore,
    ) -> &mut Self {
        self.abilities.insert(ability.into(), score);
        self.apply_ability_bonuses();
        self
    }

    /// Adds or replaces one skill.
    pub fn add_skill<S, P>(&mut self, skill: S, proficiency: P) -> &mut Self
    where
        S: Into<SkillId>,
        P: Into<Proficiency>,
    {
        self.skills.insert(skill.into(), proficiency.into());
        self
    }

    /// Applies ability bonuses to the character's statistics.
    fn apply_ability_bonuses(&mut self) {
        self.apply_constitution_bonus();
    }

    fn apply_constitution_bonus(&mut self) {
        let (_, constitution) = self
            .abilities()
            .find(|(id, _)| *id == CONSTITUTION)
            .expect("character does not have a CONSTITUTION score");
        // Compute the hit points.
        let base_hp = i32::from(self.hit_points_history.total());
        let bonus = self.hit_points_history.count() as i32 * i32::from(constitution.modifier());
        self.hit_points = HitPoints::from_value(
            std::cmp::max(base_hp + bonus, 1)
                .try_into()
                .expect("hit points > u16"),
        );
    }

    /// Spawns a character in the given battle and returns an handler to it.
    /// The character is guaranteed to be compliant with the rules contained in the current Compendium.
    ///
    /// # Errors
    ///
    /// An error is returned if the character is invalid.
    pub fn spawn<'a>(
        &'a self,
        server: &'a mut Server<SRDRules>,
    ) -> SRDResult<CreatureHandleMut<'a, Server<SRDRules>>> {
        CharacterSpawner::new(&self).spawn(server)?;
        Ok(CreatureHandleMut::new(&self.id, server))
    }
}

impl PartialEq for Character {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Character {}

impl Hash for Character {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use self::class::fighter::FIGHTER;
    use self::race::hill_dwarf::HILL_DWARF;
    use super::*;
    use crate::ability::RESERVED_ABILITIES;
    use crate::compendium::init_srd_compendium;
    use crate::skill::RESERVED_SKILLS;

    #[test]
    fn character_equality() {
        let _ = init_srd_compendium();
        let c1 = Character::new("one", HILL_DWARF, FIGHTER).unwrap();
        let c2 = Character::new("two", HILL_DWARF, FIGHTER).unwrap();
        assert_ne!(c1, c2);
        let mut c3 = Character::new("one", HILL_DWARF, FIGHTER).unwrap();
        c3.add_skill(1, true);
        assert_eq!(c1, c3);
    }

    #[test]
    fn character_has_default_abilities_skills() {
        let _ = init_srd_compendium();
        let c = Character::new("one", HILL_DWARF, FIGHTER).unwrap();
        assert_eq!(*c.id(), "one".into());
        assert_eq!(*c.race(), HILL_DWARF.into());
        assert_eq!(*c.class(), FIGHTER.into());
        assert_eq!(c.abilities().count(), RESERVED_ABILITIES.into());
        assert_eq!(c.skills().count(), RESERVED_SKILLS.into());
    }

    #[test]
    fn hit_points_calculation() {
        let _ = init_srd_compendium();
        let mut c = Character::new("one", HILL_DWARF, FIGHTER).unwrap();
        assert_eq!(c.hit_points().value(), 11);
        c.add_ability(CONSTITUTION, AbilityScore::capped(20));
        assert_eq!(c.hit_points().value(), 16);
    }
}
