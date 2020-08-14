//! Creatures proficiencies.

use serde::{Deserialize, Serialize};

/// Tells whether or not a proficiency is known.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Proficiency(pub bool);

impl From<bool> for Proficiency {
    fn from(item: bool) -> Self {
        Proficiency(item)
    }
}
