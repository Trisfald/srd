//! Actions of creatures.

use crate::error::{SRDError, SRDResult};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// An action is something that an actor is able to perform.
/// Most often an action causes a change in the state of one or more objects.\
/// Actions are identified by an id and can be of different nature.
#[derive(Debug, Clone)]
pub struct Action {
    id: ActionId,
    value: ActionValue,
}

impl Action {
    const fn new(id: ActionId, value: ActionValue) -> Self {
        Self { id, value }
    }

    /// Returns a reference to `()` if this action is of the correct type, otherwise an error.
    pub const fn movement(&self) -> SRDResult<()> {
        if let ActionValue::Movement = &self.value {
            Ok(())
        } else {
            Err(SRDError::IncorrectVariant)
        }
    }

    /// Returns a reference to `()` if this action is of the correct type, otherwise an error.
    pub const fn attack(&self) -> SRDResult<()> {
        if let ActionValue::Attack = &self.value {
            Ok(())
        } else {
            Err(SRDError::IncorrectVariant)
        }
    }
}

impl From<ActionInitializer> for Action {
    fn from(item: ActionInitializer) -> Self {
        use ActionInitializer::*;
        match item {
            Movement => Self::new(ActionId::Movement, ActionValue::Movement),
            Attack => Self::new(ActionId::Attack, ActionValue::Attack),
        }
    }
}

impl weasel::Id for Action {
    type Id = ActionId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

/// Uniquely identifies an `Action`.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ActionId {
    Movement,
    Attack,
}

/// Encapsulates the actual value of an actions.
#[derive(Debug, Clone, DisplayVariant)]
enum ActionValue {
    Movement, // TODO add a meaningful value (ft per turn)
    Attack,   // TODO add a meaningful value (?)
}

/// Initializer to create an action.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ActionInitializer {
    Movement,
    Attack,
}

/// `ActionsSeed` is used to generate all actions of an actor.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ActionsSeed {
    /// A simple list of action initializers.
    pub actions: Vec<ActionInitializer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_action() {
        let action: Action = ActionInitializer::Movement.into();
        assert!(action.movement().is_ok());
        assert!(action.attack().is_err());
    }
}
