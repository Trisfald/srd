//! Actions of creatures.

use serde::{Deserialize, Serialize};
use std::fmt;

/// An action is something that an actor is able to perform.
/// Most often an action causes a change in the state of one or more objects.\
/// Actions are identified by an id and can be of different nature.
#[derive(Debug, Clone)]
pub struct Action {
    id: ActionId,
    value: ActionValue,
}

impl Action {
    fn new(id: ActionId, value: ActionValue) -> Self {
        Self { id, value }
    }

    /// Returns a reference to `()` if this action is of the correct type, otherwise `None`.
    pub fn movement(&self) -> Option<()> {
        if let ActionValue::Movement = &self.value {
            Some(())
        } else {
            None
        }
    }

    /// Returns a reference to `()` if this action is of the correct type, otherwise `None`.
    pub fn attack(&self) -> Option<()> {
        if let ActionValue::Attack = &self.value {
            Some(())
        } else {
            None
        }
    }
}

impl From<ActionInitializer> for Action {
    fn from(item: ActionInitializer) -> Self {
        use ActionInitializer::*;
        match item {
            Movement => Action::new(ActionId::Movement, ActionValue::Movement),
            Attack => Action::new(ActionId::Attack, ActionValue::Attack),
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
#[derive(Debug, Clone)]
enum ActionValue {
    Movement, // TODO add a meaningful value (ft per turn)
    Attack,   // TODO add a meaningful value (?)
}

impl fmt::Display for ActionValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ActionValue::*;
        match self {
            Movement => write!(f, "Movement"),
            Attack => write!(f, "Attack"),
        }
    }
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
        assert!(action.movement().is_some());
        assert!(action.attack().is_none());
    }
}
