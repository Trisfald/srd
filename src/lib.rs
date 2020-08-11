#![deny(missing_docs)]
#![doc(test(attr(warn(warnings))))]

//! TODO copy pieces of the readme here

pub mod ability;
pub use self::ability::{AbilityId, AbilityScore};

pub mod character;
pub use crate::character::Character;

pub mod compendium;
pub use crate::compendium::{compendium, set_boxed_compendium, set_compendium, Compendium};

pub mod constants;

pub mod error;
pub use crate::error::{SRDError, SRDResult};

pub mod rules;

pub mod skill;
pub use self::skill::SkillId;

pub(crate) mod util;
