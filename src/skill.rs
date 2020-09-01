//! Creatures skills.

use serde::{Deserialize, Serialize};

/// Identifies a skill.
///
/// # Reference
///
/// A skill represents a specific aspect of an ability score.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct SkillId(pub u8);

impl From<u8> for SkillId {
    fn from(item: u8) -> Self {
        SkillId(item)
    }
}

/// Id of the Acrobatics skill.
///
/// # Reference
///
/// The Acrobatics skill reflects aptitude in certain kinds of Dexterity checks.
pub const ACROBATICS: SkillId = SkillId(0);

/// Id of the Animal Handling skill.
///
/// # Reference
///
/// The Animal Handling skill reflects aptitude in certain kinds of Wisdom checks.
pub const ANIMAL_HANDLING: SkillId = SkillId(1);

/// Id of the Arcana skill.
///
/// # Reference
///
/// The Arcana skill reflects aptitude in certain kinds of Intelligence checks.
pub const ARCANA: SkillId = SkillId(2);

/// Id of the Athletics skill.
///
/// # Reference
///
/// The Athletics skill reflects aptitude in certain kinds of Strength checks.
pub const ATHLETICS: SkillId = SkillId(3);

/// Id of the Deception skill.
///
/// # Reference
///
/// The Deception skill reflects aptitude in certain kinds of Charisma checks.
pub const DECEPTION: SkillId = SkillId(4);

/// Id of the History skill.
///
/// # Reference
///
/// The History skill reflects aptitude in certain kinds of Intelligence checks.
pub const HISTORY: SkillId = SkillId(5);

/// Id of the Insight skill.
///
/// # Reference
///
/// The Insight skill reflects aptitude in certain kinds of Wisdom checks.
pub const INSIGHT: SkillId = SkillId(6);

/// Id of the Intimidation skill.
///
/// # Reference
///
/// The Intimidation skill reflects aptitude in certain kinds of Charisma checks.
pub const INTIMIDATION: SkillId = SkillId(7);

/// Id of the Investigation skill.
///
/// # Reference
///
/// The Investigation skill reflects aptitude in certain kinds of Intelligence checks.
pub const INVESTIGATION: SkillId = SkillId(8);

/// Id of the Medicine skill.
///
/// # Reference
///
/// The Medicine skill reflects aptitude in certain kinds of Wisdom checks.
pub const MEDICINE: SkillId = SkillId(9);

/// Id of the Nature skill.
///
/// # Reference
///
/// The Nature skill reflects aptitude in certain kinds of Intelligence checks.
pub const NATURE: SkillId = SkillId(10);

/// Id of the Perception skill.
///
/// # Reference
///
/// The Perception skill reflects aptitude in certain kinds of Wisdom checks.
pub const PERCEPTION: SkillId = SkillId(11);

/// Id of the Performance skill.
///
/// # Reference
///
/// The Performance skill reflects aptitude in certain kinds of Charisma checks.
pub const PERFORMANCE: SkillId = SkillId(12);

/// Id of the Persuasion skill.
///
/// # Reference
///
/// The Persuasion skill reflects aptitude in certain kinds of Charisma checks.
pub const PERSUASION: SkillId = SkillId(13);

/// Id of the Religion skill.
///
/// # Reference
///
/// The Religion skill reflects aptitude in certain kinds of Intelligence checks.
pub const RELIGION: SkillId = SkillId(14);

/// Id of the Sleight of Hand skill.
///
/// # Reference
///
/// The Sleight of Hand skill reflects aptitude in certain kinds of Dexterity checks.
pub const SLEIGHT_OF_HAND: SkillId = SkillId(15);

/// Id of the Stealth skill.
///
/// # Reference
///
/// The Stealth skill reflects aptitude in certain kinds of Dexterity checks.
pub const STEALTH: SkillId = SkillId(16);

/// Id of the Survival skill.
///
/// # Reference
///
/// The Survival skill reflects aptitude in certain kinds of Wisdom checks.
pub const SURVIVAL: SkillId = SkillId(17);

/// Number of core skills.
pub const RESERVED_SKILLS: u8 = 18;

/// Returns a string representation of an `SkillId`.\
/// The string is accurate only when using the skills defined by the SRD.
pub fn srd_skill_string(id: SkillId) -> String {
    match id.0 {
        0 => "acrobatics",
        1 => "animal handling",
        2 => "arcana",
        3 => "athletics",
        4 => "deception",
        5 => "history",
        6 => "insight",
        7 => "intimidation",
        8 => "investigation",
        9 => "medicine",
        10 => "nature",
        11 => "perception",
        12 => "performance",
        13 => "persuasion",
        14 => "religion",
        15 => "sleight of hand",
        16 => "stealth",
        17 => "survival",
        _ => unimplemented!("skill id not exiting in the SRD"),
    }
    .to_string()
}
