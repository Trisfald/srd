//! weasel's battle rules implementation.

pub mod actor_rules;

pub mod character_rules;

pub mod core;

pub mod fight_rules;

pub mod narrator;

pub mod rounds_rules;

pub mod space_rules;

pub mod team_rules;

use self::actor_rules::SRDActorRules;
use self::character_rules::SRDCharacterRules;
use self::fight_rules::SRDFightRules;
use self::narrator::Narrator;
use self::rounds_rules::SRDRoundsRules;
use self::space_rules::SRDSpaceRules;
use self::team_rules::SRDTeamRules;
use crate::compendium::compendium;
use crate::util::PackageVersion;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use weasel::rules::{empty::EmptyUserRules, entropy::UniformDistribution};
use weasel::BattleRules;

/// weasel compatible battle rules implementing the Systems Reference Document (SRD).
pub struct SRDRules<N: Narrator> {
    narrator: Arc<N>,
    team_rules: SRDTeamRules<N>,
    character_rules: SRDCharacterRules<N>,
    actor_rules: SRDActorRules<N>,
    fight_rules: SRDFightRules<N>,
    user_rules: EmptyUserRules,
    space_rules: Option<SRDSpaceRules<N>>,
    rounds_rules: Option<SRDRoundsRules<N>>,
    entropy_rules: UniformDistribution<u8>,
    version: SRDRulesVersion,
}

impl<N: Narrator> SRDRules<N> {
    /// Creates a new instance of these rules.
    pub fn new(narrator: N) -> Self {
        let narrator = Arc::new(narrator);
        let instance = Self {
            narrator: narrator.clone(),
            team_rules: SRDTeamRules::new(narrator.clone()),
            character_rules: SRDCharacterRules::new(narrator.clone()),
            actor_rules: SRDActorRules::new(narrator.clone()),
            fight_rules: SRDFightRules::new(narrator.clone()),
            user_rules: EmptyUserRules::default(),
            space_rules: Some(SRDSpaceRules::new(narrator.clone())),
            rounds_rules: Some(SRDRoundsRules::new(narrator)),
            entropy_rules: UniformDistribution::default(),
            version: SRDRulesVersion::new(compendium().version(), PackageVersion::from_env()),
        };
        log::debug!("created a new instance of SRDRules");
        instance
    }

    /// Returns the narrator of the battle.
    pub fn narrator(&self) -> &N {
        &self.narrator
    }
}

impl<N: Narrator + Default> Default for SRDRules<N> {
    fn default() -> Self {
        Self::new(N::default())
    }
}

impl<N: Narrator> BattleRules for SRDRules<N> {
    type TR = SRDTeamRules<N>;
    type CR = SRDCharacterRules<N>;
    type AR = SRDActorRules<N>;
    type FR = SRDFightRules<N>;
    type UR = EmptyUserRules;
    type SR = SRDSpaceRules<N>;
    type RR = SRDRoundsRules<N>;
    // Uniform distribution of random numbers to roll from a d4 up to a d100.
    type ER = UniformDistribution<u8>;
    type Version = SRDRulesVersion;

    fn team_rules(&self) -> &Self::TR {
        &self.team_rules
    }

    fn character_rules(&self) -> &Self::CR {
        &self.character_rules
    }

    fn actor_rules(&self) -> &Self::AR {
        &self.actor_rules
    }

    fn fight_rules(&self) -> &Self::FR {
        &self.fight_rules
    }

    fn user_rules(&self) -> &Self::UR {
        &self.user_rules
    }

    fn space_rules(&mut self) -> Self::SR {
        self.space_rules.take().expect("space_rules is None!")
    }

    fn rounds_rules(&mut self) -> Self::RR {
        self.rounds_rules.take().expect("rounds_rules is None!")
    }

    fn entropy_rules(&mut self) -> Self::ER {
        self.entropy_rules
    }

    fn version(&self) -> &Self::Version {
        &self.version
    }
}

/// Version of a SRDRules instance.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct SRDRulesVersion {
    compendium: u32,
    package: PackageVersion,
}

impl SRDRulesVersion {
    pub(crate) fn new(compendium: u32, package: PackageVersion) -> Self {
        SRDRulesVersion {
            compendium,
            package,
        }
    }
}
