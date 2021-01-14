use std::fmt::Debug;

use serde::Serialize;

/// Type used to store score points
pub type Score = usize;

/// Type used to store player ID
pub type PlayerId = Id;

use crate::common::{
    clone_arc::{Clonable, CloneArc},
    id_generator::{new_id, Id},
    serializable::Serializable,
};
/// Represents player _rage_.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub enum PlayerRage {
    Red,
    Green,
    Blue,
    White,
    Black,
    Yellow,
}

const MAX_PLAYER_NAME_LEN: usize = 8usize;

#[derive(Clone, Debug, Serialize)]
/// Contains the _static_ information of a player.
pub struct PlayerInfo {
    /// Player rage
    rage: PlayerRage,
    /// Player name
    name: String,
    /// Whether this player is controlled by human
    is_human: bool,
    /// Id of the player
    id: PlayerId,
}

impl PlayerInfo {
    /// Create a new player info with provided data
    ///
    /// *Note*: Player name will be truncated to 16 chars.
    pub fn new(name: &str, rage: PlayerRage, is_human: bool) -> Self {
        Self {
            name: String::from(&name[..usize::min(name.len(), MAX_PLAYER_NAME_LEN)]),
            rage,
            is_human,
            id: new_id(),
        }
    }

    /// Returns player name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns player rage
    pub fn rage(&self) -> PlayerRage {
        self.rage
    }

    /// Returns whether player is human
    pub fn is_human(&self) -> bool {
        self.is_human
    }

    /// Returns player ID
    pub fn id(&self) -> PlayerId {
        self.id
    }
}

impl Serializable for PlayerInfo {
    /// Type of data to be serialized
    type Data = PlayerInfo;

    /// Return data to be serialized
    fn data_to_be_serialized(&self) -> &Self::Data {
        &self
    }
}

/// Represents a generic state for a player.
pub trait PlayerState
where
    Self: Debug + CloneArc,
    Self::Data: Debug + Clonable + Serializable,
{
    type Data;

    /// Returns whether the player is alive (thus can play).
    fn is_alive(&self) -> bool;

    /// Returns whether the player is currently active and playing its turn.
    fn is_current(&self) -> bool;

    /// Returns a numeric value representing player current score.
    fn score(&self) -> Score;

    /// Returns the player ID
    fn id(&self) -> PlayerId;
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::{
        common::check_for_duplicate,
        constants::NUMBER_OF_LOOPS_FOR_NORMAL_TEST,
        random_generator::{random_bool, random_number, random_string},
    };

    use super::PlayerRage;
    use super::{PlayerInfo, MAX_PLAYER_NAME_LEN};

    #[test]
    /// Test if constructor is not messing up data
    fn new_test() {
        let mut ids = vec![];
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let len = random_number(0, 32);
            let name = random_string(len);
            let is_human = random_bool();
            let rage_sel = random_number(0, 6);

            let rage = match rage_sel {
                0 => PlayerRage::Red,
                1 => PlayerRage::Green,
                2 => PlayerRage::Blue,
                3 => PlayerRage::White,
                4 => PlayerRage::Black,
                _ => PlayerRage::Yellow,
            };

            let p_info = PlayerInfo::new(&name, rage, is_human);

            assert_eq!(
                p_info.name(),
                &name[..usize::min(name.len(), MAX_PLAYER_NAME_LEN)]
            );
            assert!(p_info.name().len() <= MAX_PLAYER_NAME_LEN);
            assert_eq!(p_info.is_human(), is_human);
            assert_eq!(p_info.rage(), rage);
            ids.push(p_info.id());
        }

        assert!(check_for_duplicate(&mut ids));
    }

    #[test]
    /// Check if empty name is accepted
    fn empty_name_test() {
        let p_info = PlayerInfo::new("", PlayerRage::Red, true);
        assert_eq!(p_info.name(), "");
    }
}
