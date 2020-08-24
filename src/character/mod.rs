//! Playable and non-playable characters.

pub mod class;
pub use self::class::{ClassId, ClassModel};

pub mod level;
pub use self::level::Level;

pub mod race;
pub use self::race::{RaceId, RaceModel};

mod spawn;

use self::spawn::CharacterSpawner;
use crate::ability::{AbilityId, AbilityScore, DEFAULT_ABILITY_SCORE};
use crate::compendium::compendium;
use crate::error::{SRDError, SRDResult};
use crate::hit_points::HitPoints;
use crate::proficiency::{Proficiency, DEFAULT_PROFICIENCY};
use crate::rules::SRDRules;
use crate::skill::SkillId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        let class = class.into();
        let class_model = compendium()
            .class_model(&class)
            .ok_or(SRDError::ClassNotFound(class.clone()))?;
        let mut instance = Self {
            id: id.into(),
            race: race.into(),
            class,
            level: Level::default(),
            hit_points: HitPoints::from_value(class_model.hit_points_at_1st_level().into()),
            abilities: HashMap::new(),
            skills: HashMap::new(),
        };
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

    /// Returns an iterator over the character's abilities.
    pub fn abilities(&self) -> impl Iterator<Item = (&AbilityId, &AbilityScore)> {
        self.abilities.iter()
    }

    /// Returns an iterator over the character's skill proficiencies.
    pub fn skills(&self) -> impl Iterator<Item = (&SkillId, &Proficiency)> {
        self.skills.iter()
    }

    /// Adds or replaces one ability.
    pub fn add_ability<A: Into<AbilityId>>(
        &mut self,
        ability: A,
        score: AbilityScore,
    ) -> &mut Self {
        self.abilities.insert(ability.into(), score);
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

    /// Spawns a character in the given battle and returns an handler to it.
    /// The character is guaranteed to be compliant with the rules contained in the current Compendium.
    ///
    /// # Errors
    ///
    /// An error is returned if the character is invalid.
    pub fn spawn<P>(&self, server: &mut Server<SRDRules>) -> SRDResult<()> {
        CharacterSpawner::new(&self).spawn(server)
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
    use crate::skill::RESERVED_SKILLS;
    use crate::ability::RESERVED_ABILITIES;
    use super::*;

    #[test]
    fn character_equality() {
        let c1 = Character::new("one", HILL_DWARF, FIGHTER).unwrap();
        let c2 = Character::new("two", HILL_DWARF, FIGHTER).unwrap();
        assert_ne!(c1, c2);
        let mut c3 = Character::new("one", HILL_DWARF, FIGHTER).unwrap();
        c3.add_skill(1, true);
        assert_eq!(c1, c3);
    }

    #[test] 
    fn character_has_default_abilities_skills() {
        let c = Character::new("one", HILL_DWARF, FIGHTER).unwrap();
        assert_eq!(*c.id(), "one".into());
        assert_eq!(*c.race(), HILL_DWARF.into());
        assert_eq!(*c.class(), FIGHTER.into());
        assert_eq!(c.abilities().count(), RESERVED_ABILITIES.into());
        assert_eq!(c.skills().count(), RESERVED_SKILLS.into());
    } 
}
