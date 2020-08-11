//! Playable and non-playable characters.

pub mod class;
pub use self::class::{ClassId, ClassModel};

pub mod race;
pub use self::race::{RaceId, RaceModel};

use crate::ability::{AbilityId, AbilityScore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// A character unique identifier.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct CharacterId(pub u8);

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
    /// The character's abilities.
    pub abilities: HashMap<AbilityId, AbilityScore>,
    // bunch of skills <SkillId, Proficiency>
}

impl Character {
    /// Creates a new character.
    pub fn new(
        id: CharacterId,
        race: RaceId,
        class: ClassId,
        abilities: HashMap<AbilityId, AbilityScore>,
    ) -> Self {
        let instance = Self {
            id,
            race,
            class,
            abilities,
        };
        log::debug!("created character {:?}", id);
        instance
    }

    /// Returns the character's id.
    pub fn id(&self) -> CharacterId {
        self.id
    }

    /// Returns the character's race.
    pub fn race(&self) -> RaceId {
        self.race
    }

    /// Returns the character's class.
    pub fn class(&self) -> ClassId {
        self.class
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
    use super::*;

    #[test]
    fn equality() {
        // TODO
    }

    #[test]
    fn hash() {
        // TODO
    }
}
