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

pub mod error;
pub use crate::error::{SRDError, SRDResult};

pub mod proficiency;
pub use crate::proficiency::Proficiency;

pub mod rules;

pub mod skill;
pub use self::skill::SkillId;

pub(crate) mod util;
