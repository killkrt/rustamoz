use std::sync::Arc;

use super::{
    action::Action,
    game_state::GameState,
    player::{PlayerId, PlayerState},
};

/// Represents a generic game rule.
pub trait GameRule
where
    Self::GS: GameState,
    Self::A: Action,
{
    /// Associated type for game state
    type GS;
    /// Associated type for action
    type A;

    /// Check whether the specified action is valid in the specified game state.
    fn is_valid(&self, game_state: &Self::GS, action: &Self::A) -> bool;

    /// Execute an action on a specified game state
    ///
    /// Returns a tuple with new game state and list of actions (as a reaction for the provided action).
    fn execute(
        &self,
        game_state: Arc<Self::GS>,
        action: Arc<Self::A>,
    ) -> (Arc<Self::GS>, Vec<Arc<Self::A>>);

    /// Returns `true` if this rule can manage the provided action.
    fn can_handle(&self, action: &Self::A) -> bool;
}

pub trait BasicRules
where
    Self::GS: GameState,
    Self::PS: PlayerState,
{
    /// Associated type for game state
    type GS;
    /// Associated type for player state
    type PS;

    /// Given a `GameState` returns the winner or `None`
    fn winner(&self, game_state: &Self::GS) -> Option<PlayerId>;

    fn next_player(
        &self,
        game_state: &Self::GS,
        players: Vec<&dyn PlayerState<Data = Self::PS>>,
    ) -> Option<PlayerId>;
}
