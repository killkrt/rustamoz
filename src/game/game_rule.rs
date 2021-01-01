use super::{action::Action, game_state::GameState};

/// Represents a generic game rule.
pub trait GameRule
where
    Self::GS: GameState,
    Self::A: Action,
{
    type GS;
    type A;

    /// Check whether the specified action is valid in the specified game state.
    fn is_valid(&self, game_state: &Self::GS, action: &Self::A) -> bool;

    /// Execute an action on a specified game state
    ///
    /// Returns a tuple with new game state and list of actions (as a reaction for the provided action).
    fn execute(&self, game_state: &Self::GS, action: &Self::A) -> (Self::GS, Vec<Self::A>);

    /// Returns `true` if this rule can manage the provided action.
    fn can_handle(&self, action: &Self::A) -> bool;
}
