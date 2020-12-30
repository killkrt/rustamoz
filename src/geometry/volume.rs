use super::vector::Position;
use super::vector::{Distance, Scalar};
use log::warn;
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
                warn!(
                    "Provided corners are not valid: BLC {:?} - TRC {:?}",
                    blc, trc
                );
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

    /// Compute volume (how many _positions_ inside the volume)
    pub fn volume(&self) -> Scalar {
        let d = self.diagonal();
        d.x() * d.y() * d.z()
    }

    /// Return whether a position is inside the bounduary box
    pub fn is_inside(&self, position: &Position) -> bool {
        let diff = *position - self.bottom_left_corner;
        let diagonal = self.diagonal();

        diff.x() <= diagonal.x()
            && diff.y() <= diagonal.y()
            && diff.z() <= diagonal.z()
            && diff.is_positive()
    }
}

/// Support struct for iteration over Volume
pub struct VolumeIterator<'a> {
    current_index: Scalar,
    size: Scalar,
    volume: &'a Volume,
}

impl<'a> VolumeIterator<'a> {
    /// Create an iterator for a `Volume` and set at the start position
    fn new(volume: &'a Volume) -> Self {
        Self {
            current_index: 0,
            size: volume.volume(),
            volume,
        }
    }
}

impl<'a> Iterator for VolumeIterator<'a> {
    type Item = Position;

    /// Advance the iterator to next position of a `Volume`.
    ///
    /// Iterator begin form bottom left corner and proceed increasing
    /// x cordinates, then y, then z.
    ///
    /// Returns `None` when it reaches the top right corner.
    fn next(&mut self) -> Option<Position> {
        if self.current_index >= self.size {
            None
        } else {
            // Origin of volume
            let origin = self.volume.bottom_left_corner();
            // Diagonal
            let diagonal = self.volume.diagonal();
            // Size of volume on X and Y axis
            let x_size = diagonal.x();
            let y_size = diagonal.y();
            // Current index position for iteration
            let index = self.current_index;
            // Compute position using origin + f(index)
            let x = origin.x() + (index as Scalar % x_size);
            let y = origin.y() + ((index as Scalar / x_size) % y_size);
            let z = origin.z() + (index as Scalar / (x_size * y_size));

            // Next position
            self.current_index += 1;

            Some(Position::new(x, y, z))
        }
    }
}

impl<'a> IntoIterator for &'a Volume {
    /// Type of elements being iterated over
    type Item = Position;
    /// Type of iterator being turned into
    type IntoIter = VolumeIterator<'a>;

    /// Returns an iterator that iterate along all position inside
    /// this volume
    fn into_iter(self) -> Self::IntoIter {
        VolumeIterator::new(self)
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

    #[test]
    /// Check if volume is computed correctly
    fn volume_test() {
        // Volume of size 1 test
        assert_eq!(
            Volume::new(&Position::new(0, 0, 0), &Position::new(1, 1, 1))
                .unwrap()
                .volume(),
            1
        );

        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Create volume
            let vol = random_volume(1, 100);
            let d0 = vol.diagonal();
            assert_eq!(vol.volume(), d0.x() * d0.y() * d0.z());
        }
    }

    #[test]
    /// Check if iterator returns all the positions inside the volume
    fn iterator_test() {
        use std::collections::HashMap;

        for _ in 0..NUMBER_OF_LOOPS_FOR_SMALL_TEST {
            let bbox = random_volume(1, 10);

            // The iterator should countain all the position in the "volume"
            assert_eq!(bbox.into_iter().count(), bbox.volume() as usize);
            // All positions returned by iterator should be inside the bounding box
            assert!(bbox.into_iter().all(|pos| bbox.is_inside(&pos)));
            // There should be not duplicates
            // Usage of hash table instead of simpler vector for perfomance reason
            let mut positions = HashMap::new();
            for pos in &bbox {
                // Position should not already exist
                assert!(!positions.contains_key(&pos));
                positions.insert(pos, 0);
            }
            assert_eq!(positions.keys().len(), bbox.volume() as usize);
        }
    }

    #[test]
    /// Check if a given point is insied or outside the bounding box
    fn inside_test() {
        const SIZE: Scalar = 5;
        const DELTA: Scalar = 5;
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let blc = Position::from(random_vector(-SIZE, SIZE));
            let diagonal = Distance::from(random_vector(1, SIZE));
            let trc = blc + diagonal;
            let bb = Volume::new(&blc, &trc).unwrap();
            let blc = bb.bottom_left_corner();

            // Create all points in an area around volume with a panning of DELTA
            for x in blc.x() - DELTA..blc.x() + DELTA {
                for y in blc.y() - DELTA..blc.y() + DELTA {
                    for z in blc.z() - DELTA..blc.z() + DELTA {
                        let point = Position::new(x, y, z);

                        let d_x = point.x() - blc.x();
                        let d_y = point.y() - blc.y();
                        let d_z = point.z() - blc.z();

                        let is_inside0 = d_x <= diagonal.x()
                            && d_y <= diagonal.y()
                            && d_z <= diagonal.z()
                            && d_x >= 0
                            && d_y >= 0
                            && d_z >= 0;

                        let is_inside1 = x >= blc.x()
                            && x <= trc.x()
                            && y >= blc.y()
                            && y <= trc.y()
                            && z >= blc.z()
                            && z <= trc.z();

                        assert_eq!(is_inside0, bb.is_inside(&Position::from(point)));
                        assert_eq!(is_inside1, bb.is_inside(&Position::from(point)));
                    }
                }
            }
        }
    }
}
