use std::fmt::Debug;

use crate::common::{
    clone_arc::{Clonable, CloneArc},
    serializable::Serializable,
};

use super::actor::Actor;

/// Id for turn and turn substep
pub type TurnId = usize;

/// Represents a generic action that a `Actor` can perform or _receive_.
/// Action must be serializable and _debuggable_.
pub trait Action
where
    Self: Debug + CloneArc + Clonable,
    Self::Data: Serializable,
{
    type Data;

    /// Returns source (who has generated) of this action.
    fn source(&self) -> Actor;

    /// Returns destination (target) of this action.
    fn destination(&self) -> Actor;

    /// Turn when this action has been created.
    fn turn(&self) -> TurnId;

    /// Substep of turn when thid action has been created.
    fn turn_substep(&self) -> TurnId;
}
