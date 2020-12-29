use super::vector::Distance;
use super::vector::Position;
use serde::Serialize;

/// Represents a bounduary box/volume
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Volume {
    pub bottom_left_corner: Position,
    pub top_right_corner: Position,
    diagonal: Distance,
}

impl Volume {
    /// Create a new volume with provided corners.
    ///
    /// Volume must at least have size greater or equal to 1 in each dimension,
    /// otherwise it returns a `Option::None`.
    ///
    /// * `blc` - Bottom left corner
    /// * `trc` - Top right corner
    ///
    ///
    pub fn new(blc: &Position, trc: &Position) -> Option<Self> {
        if *blc < *trc {
            Some(Self {
                bottom_left_corner: *blc,
                top_right_corner: *trc,
                diagonal: *trc - *blc,
            })
        } else {
            None
        }
    }
}
