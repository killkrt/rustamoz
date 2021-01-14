use std::{fmt::Debug, sync::Arc};

use crate::{
    common::{clone_arc::Clonable, id_generator::Id},
    geometry::{terrain::Terrain, vector::Position},
};

use super::{action::TurnId, cell_state::CellState, player::PlayerState};
/// Represents state of particular moment of the game.
pub trait GameState
where
    Self: Debug + Clonable,
    Self::PS: PlayerState,
    Self::CS: CellState,
{
    /// Associated type for player state
    type PS;

    /// Associated type for cell state
    type CS;

    /// Returns the `Terrain` used in this game state.
    fn terrain(&self) -> Arc<Terrain>;

    /// Returns current active player.
    fn current_player(&self) -> Id;

    /// Set current active player.
    ///
    /// `true` if player ID is valid.
    fn set_current_player(&mut self, player: Id) -> bool;

    /// Returns ID of current turn being played.
    ///
    /// Every time the current player changes, the turn ID increase.
    fn current_turn(&self) -> TurnId;

    /// Set ID of current turn being played.
    fn set_current_turn(&mut self, turn: TurnId);

    /// Returns ID of substep of current turn being played.
    ///
    /// Substep can be used expecially by UI for animation purpose.
    fn current_turn_substep(&self) -> TurnId;

    /// Sets ID of substep of current turn being played.
    ///
    /// Substep can be used expecially by UI for animation purpose.
    fn set_current_turn_substep(&mut self, substep: TurnId);

    /// Returns the state of selected player.
    ///
    /// Returns `None` if player Id is not associated to any player
    /// of this game state.
    fn player_state(&self, player_id: Id) -> Option<Self::PS>;

    /// Set the state of selected player.
    ///
    /// Returns `false` if player Id is not associated to any player
    /// of this game state.
    fn set_player_state(&mut self, player_id: Id, state: Self::PS) -> bool;

    /// Returns the state of specified cell.
    ///
    /// `None` if the position is not valid.
    fn cell_state(&self, position: &Position) -> Option<Self::CS>;

    /// Sets the state of specified cell.
    ///
    /// Return `false` if the position is not valid.
    fn set_cell_state(&mut self, position: &Position, state: Self::CS) -> bool;
}
