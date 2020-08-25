//! Implementation of rules for movement and positions.

use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{PositionClaim, SpaceRules, WeaselResult, WriteMetrics};

/// Rules for creatures' movement and spatial positions.
pub struct SRDSpaceRules {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<dyn Narrator>,
}

impl SRDSpaceRules {
    /// Creates a new instance.
    pub(crate) fn new(narrator: Arc<dyn Narrator>) -> Self {
        Self { narrator }
    }
}

impl SpaceRules<SRDRules> for SRDSpaceRules {
    type Position = (); // TODO use a real type
    type SpaceSeed = (); // TODO use a real type
    type SpaceAlteration = (); // TODO use a real type
    type SpaceModel = (); // TODO use a real type

    fn generate_model(&self, _: &Option<Self::SpaceSeed>) -> Self::SpaceModel {}

    fn check_move(
        &self,
        _model: &Self::SpaceModel,
        _claim: PositionClaim<SRDRules>,
        _position: &Self::Position,
    ) -> WeaselResult<(), SRDRules> {
        Ok(())
    }

    fn move_entity(
        &self,
        _model: &mut Self::SpaceModel,
        _claim: PositionClaim<SRDRules>,
        _position: Option<&Self::Position>,
        _metrics: &mut WriteMetrics<SRDRules>,
    ) {
        // TODO
    }
}
