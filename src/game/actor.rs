use crate::common::id_generator::Id;

/// A generic actor that can perform or _receive_ an `Action`
pub enum Actor {
    /// Game controller
    Controller,
    /// Player (human or CPU)
    Player(Id),
    /// User interface
    UI,
}
