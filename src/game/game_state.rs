use std::sync::Arc;

use crate::{common::id_generator::Id, geometry::terrain::Terrain};

use super::action::TurnId;
/// Represents state of particular moment of the game.
pub trait GameState {
    /// Returns the `Terrain` used in this game state.
    fn terrain(&self) -> Arc<Terrain>;

    /// Returns current active player.
    fn current_player(&self) -> Id;

    /// Returns ID of current turn being played.
    ///
    /// Every time the current player changes, the turn ID increase.
    fn current_turn(&self) -> TurnId;

    /// Returns ID of substep of current turn being played.
    ///
    /// Substep can be used expecially by UI for animation purpose.
    fn current_turn_substep(&self) -> TurnId;
}
