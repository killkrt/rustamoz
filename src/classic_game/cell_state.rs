use crate::{
    common::{clone_arc::Clonable, serializable::Serializable},
    generic_game::{
        cell_state::{CellOccupation, CellState},
        player::PlayerId,
    },
};
use serde::Serialize;

/// State of cell in classi game
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum ClassicCellState {
    /// Empty cell
    Empty,
    /// Occupied by a player and numbers of atoms
    Occupied(PlayerId, CellOccupation),
}

impl CellState for ClassicCellState {
    type Data = Self;

    /// Returns number of elements in a cell owned by specified player.
    ///
    /// `None` if the player has not any element in this cell.
    fn player_occupation(&self, player_id: PlayerId) -> Option<CellOccupation> {
        if let ClassicCellState::Occupied(id, occupation) = *self {
            if id == player_id {
                Some(occupation)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Set number of elements in a cell owned by specified player.
    ///
    /// `true` if it has been set sucessfully
    fn set_player_occupation(&mut self, player_id: PlayerId, count: CellOccupation) -> bool {
        if count > 0 {
            *self = ClassicCellState::Occupied(player_id, count);
            true
        } else {
            false
        }
    }

    /// Returns whether this cell is not occupied by any players.
    fn is_empty(&self) -> bool {
        self == &ClassicCellState::Empty
    }
}

impl Clonable for ClassicCellState {}

impl Serializable for ClassicCellState {
    type Data = Self;

    /// Return data to be serialized (itself)
    fn data_to_be_serialized(&self) -> &Self::Data {
        &self
    }
}

#[cfg(test)]
mod tests {
    use super::ClassicCellState;
    use crate::{
        common::clone_arc::{ArcClone, Clonable},
        generic_game::cell_state::CellState,
        test_utilities::{
            constants::NUMBER_OF_LOOPS_FOR_NORMAL_TEST,
            random_generator::{random_bool, random_number},
        },
    };

    /// Check if empty cell is reported as empty and viceversa
    #[test]
    fn empty_test() {
        let empty = ClassicCellState::Empty;
        assert!(empty.is_empty());

        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let id = random_number(0, usize::MAX);
            let occ = random_number(1, u8::MAX);
            let occupied = ClassicCellState::Occupied(id, occ);
            assert!(!occupied.is_empty());
        }
    }

    /// Check if occupied states is reported correctly
    #[test]
    fn occupation_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let id = random_number(0, usize::MAX);
            let occ = random_number(0, 10);
            let occupied = ClassicCellState::Occupied(id, occ);
            assert!(!occupied.is_empty());
            assert_eq!(occupied.player_occupation(id), Some(occ));

            for i in 0..1000 {
                if i == id {
                    assert_eq!(occupied.player_occupation(id), None);
                }
            }
        }

        let empty = ClassicCellState::Empty;
        for id in 0..100000 {
            assert_eq!(empty.player_occupation(id), None);
        }
    }

    #[test]
    /// Check if setting occupation of a cell change its occupation state
    fn set_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let mut empty = ClassicCellState::Empty;
            assert!(empty.is_empty());
            let id = random_number(0, usize::MAX);
            assert!(!empty.set_player_occupation(id, 0));
            for count in 1..10 {
                assert!(empty.set_player_occupation(id, count));
                assert!(!empty.is_empty());
                assert_eq!(empty.player_occupation(id), Some(count));

                for i in 0..1000 {
                    if i == id {
                        assert_eq!(empty.player_occupation(id), None);
                    }
                }
            }
        }
    }

    fn random_cell_state() -> ClassicCellState {
        if random_bool() {
            let id = random_number(0, usize::MAX);
            let occ = random_number(1, u8::MAX);
            ClassicCellState::Occupied(id, occ)
        } else {
            ClassicCellState::Empty
        }
    }

    #[test]
    /// Test if arc_clone works correctly
    fn arc_clone_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let cs = random_cell_state();
            let arc = cs.arc_clone();
            assert_eq!(cs, *arc);
        }
    }
}
