use super::vector::Distance;
use super::vector::Position;
use serde::Serialize;
use std::cmp::Ordering;

/// Represents a bounduary box/volume
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Volume {
    bottom_left_corner: Position,
    top_right_corner: Position,
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
    pub fn new(blc: &Position, trc: &Position) -> Option<Self> {
        match blc.partial_cmp(trc) {
            Some(Ordering::Less) =>
            // Bottom left corner is in a valid position
            {
                Some(Self {
                    bottom_left_corner: *blc,
                    top_right_corner: *trc,
                    diagonal: *trc - *blc,
                })
            }
            _ =>
            // Invalid relative corners position
            {
                None
            }
        }
    }

    /// Returns diagonal of volume (distance between top right corner and bottom left corner)
    pub fn diagonal(&self) -> Distance {
        self.diagonal
    }

    /// Get bottom left corner
    pub fn bottom_left_corner(&self) -> Position {
        self.bottom_left_corner
    }

    /// Get top right corner
    pub fn top_right_corner(&self) -> Position {
        self.top_right_corner
    }
}

#[cfg(test)]
mod tests {
    use super::super::vector::Vector;
    use super::*;
    use crate::test_utilities::random_generator::*;
    use crate::test_utilities::{constants::*, random_generator};

    #[test]
    // Test is `new` method check properly for provided arguments
    fn new_test() {
        let vectors = [
            [1, 1, 1],    // eq
            [0, 0, 0],    // less
            [0, 0, 1],    // none
            [2, 2, 2],    // great
            [1, 2, 2],    // none
            [-1, -1, -1], // less
            [1, -1, 2],   // none
            [1, 1, 2],    // none
            [2, 5, 2],    // great
        ];

        for components in &vectors {
            // Vector 0 components
            let x0 = 1;
            let y0 = 1;
            let z0 = 1;
            let v0 = Vector::new(x0, y0, z0);
            // Vector 1 components
            let x1 = components[0];
            let y1 = components[1];
            let z1 = components[2];
            let v1 = Vector::new(x1, y1, z1);

            if x0 == x1 && y0 == y1 && z0 == z1 {
                assert_eq!(v0.partial_cmp(&v1), Some(Ordering::Equal));
                assert_eq!(Volume::new(&v0, &v1), None);
            } else {
                if x0 < x1 && y0 < y1 && z0 < z1 {
                    assert_eq!(v0.partial_cmp(&v1), Some(Ordering::Less));
                    let vol = Volume::new(&v0, &v1);
                    assert_ne!(vol, None);
                    assert_eq!(vol.unwrap().top_right_corner(), v1);
                    assert_eq!(vol.unwrap().bottom_left_corner(), v0);
                } else {
                    if x0 > x1 && y0 > y1 && z0 > z1 {
                        assert_eq!(v0.partial_cmp(&v1), Some(Ordering::Greater));
                        assert_eq!(Volume::new(&v0, &v1), None);
                    } else {
                        assert_eq!(v0.partial_cmp(&v1), None);
                        assert_eq!(Volume::new(&v0, &v1), None);
                    }
                }
            }
        }

        // Random vectors
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let v0 = random_generator::random_vector(-100, 100);
            let v1 = random_generator::random_vector(-100, 100);

            let vol = Volume::new(&v0, &v1);

            assert_eq!(vol.is_some(), v0.partial_cmp(&v1) == Some(Ordering::Less));
        }
    }

    #[test]
    /// Check if volume diagonal is computer correctly
    fn diagonal_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let vol = random_volume(1, 100);
            assert_eq!(
                vol.top_right_corner() - vol.bottom_left_corner(),
                vol.diagonal()
            );
            // Diagonal must be always great than (0,0,0)
            assert!(vol.diagonal.partial_cmp(&Vector::zero()) == Some(Ordering::Greater));
        }
    }
}
