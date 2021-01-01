use serde::Serialize;
use std::fmt::Debug;

use crate::common::id_generator::Id;

/// Type used for number of elements in a cell
type CellOccupation = u8;

/// Represents a generic state of a game cell.
pub trait CellState: Debug + Clone + Serialize {
    /// Returns number of elements in a cell owned by specified player.
    ///
    /// `None` if the player has not any element in this cell.
    fn player_occupation(&self, player_id: Id) -> Option<CellOccupation>;

    /// Set number of elements in a cell owned by specified player.
    ///
    /// `true` if it has been set sucessfully
    fn set_player_occupation(&mut self, player_id: Id, count: CellOccupation) -> bool;

    /// Returns whether this cell is not occupied by any players.
    fn is_empty(&self) -> bool;
}
