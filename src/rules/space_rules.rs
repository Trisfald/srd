//! Implementation of rules for movement and positions.

use crate::rules::narrator::Narrator;
use crate::rules::SRDRules;
use std::sync::Arc;
use weasel::{PositionClaim, SpaceRules, WeaselResult, WriteMetrics};

/// Rules for creatures' movement and spatial positions.
pub struct SRDSpaceRules<N: Narrator> {
    #[allow(dead_code)] // TODO remove
    narrator: Arc<N>,
}

impl<N: Narrator> SRDSpaceRules<N> {
    /// Creates a new instance.
    pub(crate) fn new(narrator: Arc<N>) -> Self {
        Self { narrator }
    }
}

impl<N: Narrator> SpaceRules<SRDRules<N>> for SRDSpaceRules<N> {
    type Position = (); // TODO use a real type
    type SpaceSeed = (); // TODO use a real type
    type SpaceAlteration = (); // TODO use a real type
    type SpaceModel = (); // TODO use a real type

    fn generate_model(&self, _: &Option<Self::SpaceSeed>) -> Self::SpaceModel {
        unimplemented!()
    }

    fn check_move(
        &self,
        _model: &Self::SpaceModel,
        _claim: PositionClaim<SRDRules<N>>,
        _position: &Self::Position,
    ) -> WeaselResult<(), SRDRules<N>> {
        unimplemented!()
    }

    fn move_entity(
        &self,
        _model: &mut Self::SpaceModel,
        _claim: PositionClaim<SRDRules<N>>,
        _position: Option<&Self::Position>,
        _metrics: &mut WriteMetrics<SRDRules<N>>,
    ) {
        unimplemented!()
    }
}
