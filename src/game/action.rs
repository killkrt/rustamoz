use std::fmt::Debug;

use serde::Serialize;

use super::actor::Actor;

/// Id for turn and turn substep
pub type TurnId = usize;

/// Represents a generic action that a `Actor` can perform or _receive_.
/// Action must be serializable and _debuggable_.
pub trait Action
where
    Self: Serialize + Debug,
{
    /// Returns source (who has generated) of this action.
    fn source(&self) -> Actor;
    /// Returns destination (target) of this action.
    fn destination(&self) -> Actor;
    /// Turn when this action has been created.
    fn turn(&self) -> TurnId;
    /// Substep of turn when thid action has been created.
    fn turn_substep(&self) -> TurnId;
}
