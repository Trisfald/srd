//! This module contains creature's speed implementation

/// Speed in feet per round
pub type SpeedValue = u16;

///
/// # Reference
///
/// Every character and monster has a speed which is the distance in feet
/// that the character or monster can walk in 1 round. This number assumes
/// short bursts of energetic movement in the midst of a life-threatening
/// situation.
pub struct Speed {
    /// Creature's base speed
    base_value: SpeedValue,
}

///
/// # Reference
/// The travel speeds given in the Travel Pace table assume relatively simple
/// terrain: roads, open plains or clear dungeon corridors. But adventurers often
/// face dense forests, deep swamps, rubble-filled ruins, steep mountains
/// and ice-covered ground - all considered difficult terrain.
///
pub enum Conditions {
    /// Terrain which is considered simple
    Normal,
    /// Terrain which is considered difficult
    DifficultTerrain,
}

///
/// # Reference
/// Movement through dangerous dungeons or wilderness areas often involves
/// more than simply walking. Adventurers might have to climb, crawl or jump
/// to get where they need to go.
///
/// Caveat: Jumping is excluded for now as it includes strength / athletic checks
pub enum MovementType {
    /// The character is climbing.
    Climb,
    /// The character is crawling.
    Crawl,
    /// The character is swimming.
    Swim,
    /// The character is just walking.
    Walk,
}

impl Speed {
    /// Creates `Speed` from a given `SpeedValue`.
    pub fn from(value: SpeedValue) -> Self {
        Self { base_value: value }
    }

    /// Calculates movement speed based on movement type and the conditions.
    fn move_speed(&self, movement_type: MovementType, conditions: Conditions) -> SpeedValue {
        self.base_value
            / (1 + Self::movement_type_penalty(movement_type)
                + Self::conditions_penalty(conditions))
    }

    fn conditions_penalty(conditions: Conditions) -> SpeedValue {
        match conditions {
            Conditions::Normal => 0,
            Conditions::DifficultTerrain => 1,
        }
    }

    fn movement_type_penalty(movement_type: MovementType) -> SpeedValue {
        match movement_type {
            MovementType::Walk => 0,
            MovementType::Swim | MovementType::Crawl | MovementType::Climb => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn easy_stroll() {
        let speed = Speed::from(5);
        assert_eq!(speed.move_speed(MovementType::Walk, Conditions::Normal), 5)
    }

    #[test]
    fn mud_walk() {
        let speed = Speed::from(5);
        assert_eq!(
            speed.move_speed(MovementType::Walk, Conditions::DifficultTerrain),
            2
        )
    }

    #[test]
    fn just_a_climb() {
        let speed = Speed::from(5);
        assert_eq!(speed.move_speed(MovementType::Climb, Conditions::Normal), 2)
    }

    #[test]
    fn crawl_to_mordor() {
        let speed = Speed::from(5);
        assert_eq!(
            speed.move_speed(MovementType::Crawl, Conditions::DifficultTerrain),
            1
        )
    }
}
