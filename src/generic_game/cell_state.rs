use std::fmt::Debug;

use super::player::PlayerId;
use crate::common::{clone_arc::Clonable, serializable::Serializable};

/// Type used for number of elements in a cell
pub type CellOccupation = u8;

/// Represents a generic state of a game cell.
pub trait CellState
where
    Self: Debug + Clonable,
    Self::Data: Serializable,
{
    type Data;

    /// Returns number of elements in a cell owned by specified player.
    ///
    /// `None` if the player has not any element in this cell.
    fn player_occupation(&self, player_id: PlayerId) -> Option<CellOccupation>;

    /// Set number of elements in a cell owned by specified player.
    ///
    /// `true` if it has been set sucessfully
    fn set_player_occupation(&mut self, player_id: PlayerId, count: CellOccupation) -> bool;

    /// Returns whether this cell is not occupied by any players.
    fn is_empty(&self) -> bool;
}
