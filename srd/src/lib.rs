#![deny(missing_docs)]
#![doc(test(attr(warn(warnings))))]

//! TODO copy pieces of the readme here

#[macro_use]
extern crate derive_new;

pub mod ability;
pub use self::ability::{AbilityId, AbilityScore};

pub mod character;
pub use crate::character::Character;

pub mod compendium;
pub use crate::compendium::{
    compendium, init_srd_compendium, set_boxed_compendium, set_compendium, Compendium,
};

pub mod constants;

pub mod dice;
pub use crate::dice::{Dice, DicePool, DiceRoll, DiceRolls, Die};

pub mod error;
pub use crate::error::{SRDError, SRDResult};

pub mod handle;
pub use crate::handle::creature_handle::CreatureHandle;

pub mod hit_points;
pub use crate::hit_points::{HitDice, HitPoints};

pub mod proficiency;
pub use crate::proficiency::{Proficiency, ProficiencyBonus};

pub mod rules;
pub use crate::rules::{narrator::Narrator, SRDRules, SRDRulesVersion};

pub mod skill;
pub use self::skill::SkillId;

pub mod util;

pub mod value;
