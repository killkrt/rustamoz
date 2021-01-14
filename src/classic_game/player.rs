use crate::{
    common::{clone_arc::Clonable, serializable::Serializable},
    generic_game::player::{PlayerId, PlayerState, Score},
};
use serde::Serialize;

/// Represents the state of a player in the classic atomz game
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct ClassicPlayerState {
    is_alive: bool,
    is_current: bool,
    score: Score,
    id: PlayerId,
}

impl Serializable for ClassicPlayerState {
    type Data = ClassicPlayerState;

    /// Returns a reference to the object itself, since it is serializable
    fn data_to_be_serialized(&self) -> &Self::Data {
        &self
    }
}

impl ClassicPlayerState {
    /// Create a new player state with default values:
    /// - Is not alive
    /// - Is not current
    /// - Score 0
    fn new(id: PlayerId) -> Self {
        Self {
            is_alive: false,
            is_current: false,
            score: 0,
            id: id,
        }
    }
}

impl Clonable for ClassicPlayerState {}

impl PlayerState for ClassicPlayerState {
    type Data = ClassicPlayerState;

    /// Returns whether the player is alive (thus can play).
    fn is_alive(&self) -> bool {
        self.is_alive
    }

    /// Returns whether the player is currently active and playing its turn.
    fn is_current(&self) -> bool {
        self.is_current
    }

    /// Returns a numeric value representing player current score.
    fn score(&self) -> Score {
        self.score
    }
    /// Returns whether the player is currently active and playing its turn.
    fn id(&self) -> PlayerId {
        self.id
    }

    /// Set whether the player is alive (thus can play).
    fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }

    /// Set whether the player is currently active and playing its turn.
    fn set_is_current(&mut self, is_current: bool) {
        self.is_current = is_current;
    }

    /// Set a numeric value representing player current score.
    fn set_score(&mut self, score: Score) {
        self.score = score;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::clone_arc::{ArcClone, Clonable},
        generic_game::player::PlayerState,
        test_utilities::{
            constants::NUMBER_OF_LOOPS_FOR_NORMAL_TEST,
            random_generator::{random_bool, random_number},
        },
    };

    use super::ClassicPlayerState;

    fn random_player_state() -> ClassicPlayerState {
        let id = random_number(0, usize::MAX);
        let mut default = ClassicPlayerState::new(id);
        let is_alive = random_bool();
        default.set_is_alive(is_alive);

        let is_current = random_bool();
        default.set_is_current(is_current);

        let score = random_number(0, u32::MAX);
        default.set_score(score);

        default
    }

    #[test]
    fn set_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let id = random_number(0, usize::MAX);
            let mut default = ClassicPlayerState::new(id);

            assert_eq!(default.is_alive(), false);
            assert_eq!(default.is_current(), false);
            assert_eq!(default.score(), 0);

            let is_alive = random_bool();
            default.set_is_alive(is_alive);
            assert_eq!(default.is_alive(), is_alive);

            let is_current = random_bool();
            default.set_is_current(is_current);
            assert_eq!(default.is_current(), is_current);

            let score = random_number(0, u32::MAX);
            default.set_score(score);
            assert_eq!(default.score(), score);

            assert_eq!(default.id(), id);
        }
    }

    #[test]
    fn arc_clone_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let ps = random_player_state();
            let arc = ps.arc_clone();
            assert_eq!(ps, *arc);
        }
    }
}
