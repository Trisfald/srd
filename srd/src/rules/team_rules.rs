//! Implementation of rules for teams.

use crate::error::SRDResult;
use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{BattleController, CreateTeam, EventTrigger, Server, TeamRules};

pub(crate) const GLOBAL_TEAM_ID: u8 = 0;

/// Teams do not exists per se in the SRD, but weasel requires them.\
/// The easy solution is have one team for all creatures.\
/// There's no need to implement any specific behavior for teams.
pub struct SRDTeamRules {
    #[allow(dead_code)]
    narrator: Arc<dyn Narrator>,
}

impl SRDTeamRules {
    /// Creates a new instance.
    pub(crate) fn new(narrator: Arc<dyn Narrator>) -> Self {
        Self { narrator }
    }
}

impl TeamRules<SRDRules> for SRDTeamRules {
    type Id = u8;
    type ObjectivesSeed = ();
    type Objectives = ();
}

/// Creates the global team if it does not exist yet.
pub(crate) fn create_global_team(server: &mut Server<SRDRules>) -> SRDResult<()> {
    if server.battle().entities().teams().count() == 0 {
        CreateTeam::trigger(server, GLOBAL_TEAM_ID).fire()?;
        log::debug!("created global team");
    }
    Ok(())
}
