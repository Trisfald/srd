//! Classic implementation of the compendium.

use crate::ability::*;
use crate::character::class::*;
use crate::character::race::*;
use crate::skill::*;
use crate::Compendium;
use std::collections::{HashMap, HashSet};

/// Classic implementation of the compendium.
/// It allows registering new components at runtime.
pub struct StandardCompendium {
    abilities: HashSet<AbilityId>,
    skills: HashMap<SkillId, AbilityId>,
    races: HashMap<RaceId, Box<dyn RaceModel>>,
    classes: HashMap<ClassId, Box<dyn ClassModel>>,
    version: u32,
}

impl StandardCompendium {
    fn empty() -> Self {
        Self {
            abilities: HashSet::new(),
            skills: HashMap::new(),
            races: HashMap::new(),
            classes: HashMap::new(),
            version: 0,
        }
    }

    /// Constructs a new `StandardCompendium` with all modules from the SRD.
    pub fn with_srd() -> Self {
        let mut compendium = Self::empty();
        compendium
            .add_srd_abilities()
            .add_srd_skills()
            .add_srd_races()
            .add_srd_classes();
        log::debug!("created a standard compendium with the core SRD modules");
        compendium
    }

    /// Sets the version of this compendium.
    pub fn set_version(&mut self, version: u32) {
        self.version = version
    }

    /// Adds a new ability.
    pub fn add_ability<T: Into<AbilityId>>(&mut self, ability: T) -> &mut Self {
        self.abilities.insert(ability.into());
        self
    }

    /// Adds all abilities from the SRD.
    pub fn add_srd_abilities(&mut self) -> &mut Self {
        self.add_ability(STRENGTH)
            .add_ability(DEXTERITY)
            .add_ability(CONSTITUTION)
            .add_ability(INTELLIGENCE)
            .add_ability(WISDOM)
            .add_ability(CHARISMA)
    }

    /// Adds or replaces a skill and the ability to which it's connected.
    pub fn add_skill<T: Into<SkillId>, D: Into<AbilityId>>(
        &mut self,
        skill: T,
        dependency: D,
    ) -> &mut Self {
        self.skills.insert(skill.into(), dependency.into());
        self
    }

    /// Adds all skills from the SRD.
    pub fn add_srd_skills(&mut self) -> &mut Self {
        self.add_skill(ACROBATICS, DEXTERITY)
            .add_skill(ANIMAL_HANDLING, WISDOM)
            .add_skill(ARCANA, INTELLIGENCE)
            .add_skill(ATHLETICS, STRENGTH)
            .add_skill(DECEPTION, CHARISMA)
            .add_skill(HISTORY, INTELLIGENCE)
            .add_skill(INSIGHT, WISDOM)
            .add_skill(INTIMIDATION, CHARISMA)
            .add_skill(MEDICINE, WISDOM)
            .add_skill(NATURE, INTELLIGENCE)
            .add_skill(PERCEPTION, WISDOM)
            .add_skill(PERFORMANCE, CHARISMA)
            .add_skill(PERSUASION, CHARISMA)
            .add_skill(RELIGION, INTELLIGENCE)
            .add_skill(SLEIGHT_OF_HAND, DEXTERITY)
            .add_skill(STEALTH, DEXTERITY)
            .add_skill(SURVIVAL, WISDOM)
    }

    /// Adds or replaces a character race and its model.
    pub fn add_race<T: Into<RaceId>>(&mut self, race: T, model: Box<dyn RaceModel>) -> &mut Self {
        self.races.insert(race.into(), model);
        self
    }

    /// Adds all character races from the SRD.
    pub fn add_srd_races(&mut self) -> &mut Self {
        self.add_race(HILL_DWARF, Box::new(hill_dwarf::hill_dwarf_model()))
    }

    /// Adds or replaces a character class and its model.
    pub fn add_class<T: Into<ClassId>>(
        &mut self,
        class: T,
        model: Box<dyn ClassModel>,
    ) -> &mut Self {
        self.classes.insert(class.into(), model);
        self
    }

    /// Adds all character classes from the SRD.
    pub fn add_srd_classes(&mut self) -> &mut Self {
        self.add_class(FIGHTER, Box::new(fighter::FighterModel::default()))
    }
}

impl Default for StandardCompendium {
    /// Constructs a new `StandardCompendium` without any registered module.
    fn default() -> Self {
        let compendium = Self::empty();
        log::debug!("created an empty standard compendium");
        compendium
    }
}

impl Compendium for StandardCompendium {
    fn version(&self) -> u32 {
        self.version
    }

    fn abilities<'a>(&'a self) -> Box<dyn Iterator<Item = &AbilityId> + 'a> {
        Box::new(self.abilities.iter())
    }

    fn skills<'a>(&'a self) -> Box<dyn Iterator<Item = &SkillId> + 'a> {
        Box::new(self.skills.keys())
    }

    fn associated_ability(&self, skill: &SkillId) -> Option<&AbilityId> {
        self.skills.get(skill)
    }

    fn races<'a>(&'a self) -> Box<dyn Iterator<Item = &RaceId> + 'a> {
        Box::new(self.races.keys())
    }

    fn race_model(&self, race: &RaceId) -> Option<&dyn RaceModel> {
        self.races.get(race).map(|e| &**e)
    }

    fn classes<'a>(&'a self) -> Box<dyn Iterator<Item = &ClassId> + 'a> {
        Box::new(self.classes.keys())
    }

    fn class_model(&self, class: &ClassId) -> Option<&dyn ClassModel> {
        self.classes.get(class).map(|e| &**e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ability::{DEXTERITY, RESERVED_ABILITIES};
    use crate::character::class::FIGHTER;
    use crate::character::race::HILL_DWARF;
    use crate::skill::{SkillId, ACROBATICS, RESERVED_SKILLS};
    use crate::{compendium, init_srd_compendium, Compendium};
    use std::thread;

    #[test]
    fn default_standard_compendium_is_empty() {
        let compendium = StandardCompendium::default();
        assert_eq!(compendium.abilities().count(), 0);
        assert_eq!(compendium.skills().count(), 0);
        assert_eq!(compendium.races().count(), 0);
        assert_eq!(compendium.classes().count(), 0);
    }

    #[test]
    fn standard_compendium_has_all_core_rules() {
        let compendium = StandardCompendium::with_srd();
        assert_eq!(compendium.abilities().count(), RESERVED_ABILITIES.into());
        assert_eq!(compendium.skills().count(), RESERVED_SKILLS.into());
        // TODO enable when we have all races and classes
        // assert_eq!(compendium.races().count(), RESERVED_RACES.into());
        // assert_eq!(compendium.classes().count(), RESERVED_CLASSES.into());
    }

    #[test]
    fn standard_compendium_multithreaded_access() {
        let _ = init_srd_compendium();
        let child = thread::spawn(move || {
            assert_eq!(compendium().abilities().count(), RESERVED_ABILITIES.into());
        });
        assert_eq!(compendium().abilities().count(), RESERVED_ABILITIES.into());
        child.join().unwrap();
    }

    #[test]
    fn standard_compendium_returns_correct_associated_ability() {
        let compendium = StandardCompendium::with_srd();
        assert_eq!(compendium.associated_ability(&ACROBATICS), Some(&DEXTERITY));
        assert_eq!(
            compendium.associated_ability(&SkillId(RESERVED_SKILLS + 1)),
            None
        );
    }

    #[test]
    fn standard_compendium_returns_correct_race_model() {
        let compendium = StandardCompendium::with_srd();
        assert!(compendium.race_model(&HILL_DWARF.into()).is_some());
        assert!(compendium.race_model(&"test".into()).is_none());
    }

    #[test]
    fn standard_compendium_returns_correct_class_model() {
        let compendium = StandardCompendium::with_srd();
        assert!(compendium.class_model(&FIGHTER.into()).is_some());
        assert!(compendium.class_model(&"test".into()).is_none());
    }
}
