//! Statistics of creatures.

use crate::ability::{AbilityId, AbilityScore};
use crate::character::{class::ClassId, level::Level, race::RaceId};
use crate::error::{SRDError, SRDResult};
use crate::hit_points::HitPoints;
use crate::proficiency::{Proficiency, ProficiencyBonus};
use crate::rules::core::size::CreatureSize;
use crate::skill::SkillId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

macro_rules! accessor {
    ($name:ident, $variant:ident, $type:ident) => {
        /// Returns a reference to `$type` if this statistic is of the correct type, otherwise an error.
        pub const fn $name(&self) -> SRDResult<&$type> {
            if let StatisticValue::$variant(value) = &self.value {
                Ok(value)
            } else {
                Err(SRDError::IncorrectVariant)
            }
        }
    };
}

/// A statistic represents a specific property of a creature.\
/// Statistics are identified by an id and can contain different kinds of values.
#[derive(Debug, Clone)]
pub struct Statistic {
    id: StatisticId,
    value: StatisticValue,
}

impl Statistic {
    const fn new(id: StatisticId, value: StatisticValue) -> Self {
        Self { id, value }
    }

    accessor! {race, Race, RaceId}

    accessor! {class, Class, ClassId}

    accessor! {level, Level, Level}

    accessor! {hit_points, HitPoints, HitPoints}

    accessor! {size, Size, CreatureSize}

    accessor! {ability, Ability, AbilityScore}

    accessor! {skill, Skill, Proficiency}

    accessor! {proficiency_bonus, ProficiencyBonus, ProficiencyBonus}

    /// Applies a change on this statistic. The change will have an effect only if its
    /// type matches the statistic's one.
    #[allow(dead_code)] // TODO remove when used
    pub(crate) fn apply_change(&mut self, change: &StatisticChange) {
        self.value.apply_change(change);
    }
}

impl From<StatisticInitializer> for Statistic {
    fn from(item: StatisticInitializer) -> Self {
        use StatisticInitializer::*;
        match item {
            Race(value) => Self::new(StatisticId::Race, StatisticValue::Race(value)),
            Class(value) => Self::new(StatisticId::Class, StatisticValue::Class(value)),
            Level(value) => Self::new(StatisticId::Level, StatisticValue::Level(value)),
            HitPoints(value) => Self::new(StatisticId::HitPoints, StatisticValue::HitPoints(value)),
            Size(value) => Self::new(StatisticId::Size, StatisticValue::Size(value)),
            Ability(id, value) => {
                Self::new(StatisticId::Ability(id), StatisticValue::Ability(value))
            }
            Skill(id, value) => Self::new(StatisticId::Skill(id), StatisticValue::Skill(value)),
            ProficiencyBonus(value) => Self::new(
                StatisticId::ProficiencyBonus,
                StatisticValue::ProficiencyBonus(value),
            ),
        }
    }
}

impl weasel::Id for Statistic {
    type Id = StatisticId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// Uniquely identifies a `Statistic`.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum StatisticId {
    Race,
    Class,
    Level,
    HitPoints,
    Size,
    Ability(AbilityId),
    Skill(SkillId),
    ProficiencyBonus,
}

/// Encapsulates the actual value of a statistic.
#[derive(Debug, Clone, DisplayVariant)]
enum StatisticValue {
    Race(RaceId),
    Class(ClassId),
    Level(Level),
    HitPoints(HitPoints),
    Size(CreatureSize),
    Ability(AbilityScore),
    Skill(Proficiency),
    ProficiencyBonus(ProficiencyBonus),
}

impl StatisticValue {
    fn apply_change(&mut self, change: &StatisticChange) {
        let mut successful = false;
        use StatisticValue::*;
        match change {
            StatisticChange::HitPoints(change) => {
                if let HitPoints(value) = self {
                    let change_abs = change.abs() as u16;
                    if *change > 0 {
                        value.add(change_abs);
                    } else {
                        value.subtract(change_abs);
                    }
                    successful = true;
                }
            }
            StatisticChange::Ability(_, change) => {
                if let Ability(value) = self {
                    let change_abs = change.abs() as u8;
                    if *change > 0 {
                        value.add(change_abs);
                    } else {
                        value.subtract(change_abs);
                    }
                    successful = true;
                }
            }
        }
        if !successful {
            log::warn!(
                "ignoring change ({}) because value ({}) doesn't match the expected variant",
                change,
                self
            );
        }
    }
}

/// Initializer to create a statistic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum StatisticInitializer {
    Race(RaceId),
    Class(ClassId),
    Level(Level),
    HitPoints(HitPoints),
    Size(CreatureSize),
    Ability(AbilityId, AbilityScore),
    Skill(SkillId, Proficiency),
    ProficiencyBonus(ProficiencyBonus),
}

/// `StatisticsSeed` is used to generate all statistics of a creature.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsSeed {
    /// A simple list of statistic initializers.
    pub statistics: Vec<StatisticInitializer>,
}

/// Encapsulates a change to a statistic.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, DisplayVariant)]
pub enum StatisticChange {
    /// A numeric change to hit points.
    HitPoints(i16),
    /// A numeric change to an ability score.
    Ability(AbilityId, i8),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_statistic() {
        let stat: Statistic = StatisticInitializer::HitPoints(HitPoints::from_value(10)).into();
        assert_eq!(stat.hit_points().unwrap().value(), 10);
    }

    #[test]
    fn change_statistic() {
        let mut stat: Statistic = StatisticInitializer::HitPoints(HitPoints::from_value(10)).into();
        assert_eq!(stat.hit_points().unwrap().value(), 10);
        stat.apply_change(&StatisticChange::HitPoints(-4));
        assert_eq!(stat.hit_points().unwrap().value(), 6);
    }
}
