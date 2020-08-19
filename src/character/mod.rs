//! Playable and non-playable characters.

pub mod class;
pub use self::class::{ClassId, ClassModel};

pub mod level;
pub use self::level::Level;

pub mod race;
pub use self::race::{RaceId, RaceModel};

use crate::ability::{AbilityId, AbilityScore};
use crate::proficiency::Proficiency;
use crate::skill::SkillId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// A character unique identifier.
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
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
    /// The character's abilities.
    pub abilities: HashMap<AbilityId, AbilityScore>,
    /// The character's proficiency in skills.
    pub skills: HashMap<SkillId, Proficiency>,
}

impl Character {
    /// Constructs a new `Character`.
    pub fn new<I, R, C>(id: I, race: R, class: C) -> Self
    where
        I: Into<CharacterId>,
        R: Into<RaceId>,
        C: Into<ClassId>,
    {
        let instance = Self {
            id: id.into(),
            race: race.into(),
            class: class.into(),
            level: Level::default(),
            abilities: HashMap::new(),
            skills: HashMap::new(),
        };
        log::debug!("created character {:?}", instance.id);
        instance
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
    pub fn spawn() {
        // TODO
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

    #[test]
    fn equality() {
        let c1 = Character::new("one", HILL_DWARF, FIGHTER);
        let c2 = Character::new("two", HILL_DWARF, FIGHTER);
        assert_ne!(c1, c2);
        let mut c3 = Character::new("one", HILL_DWARF, FIGHTER);
        c3.add_skill(1, true);
        assert_eq!(c1, c3);
    }
}
