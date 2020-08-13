//! Creatures proficiencies.

use serde::{Deserialize, Serialize};

/// Tells whether or not a proficiency is known.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub struct Proficiency(pub bool);
