//! Smart storytelling module.

/// A narrator tells of all episodes happening during the battle.
pub trait Narrator: Send + Sync {
    // TODO fn episode(episode);
}

/// A narrator that does nothing.
#[derive(Default)]
pub struct NopNarrator {}

impl Narrator for NopNarrator {}

/// A narrator to debug episodes.
#[derive(Default)]
pub struct DebugNarrator {}

impl Narrator for DebugNarrator {}
