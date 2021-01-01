#![allow(dead_code)]

use super::{vector::Position, volume::Volume};
use log::warn;
use serde::ser::{SerializeSeq, Serializer};
use serde::Serialize;
use std::collections::{hash_map::Iter, HashMap};

/// Possible types of cell materials
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum CellMaterial {
    Water,
    Ground,
}

/// Possible types of cell
#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum CellType {
    /// Cell full of material (cannot placed anything on top of that)
    Fill(CellMaterial),
    /// Flat cell (stuff can be placed on top of that)
    Flat(CellMaterial),
}

/// Represents a terrain where players can place their pawns
///
/// Terrain has a volume (its bounding box), cells can be placed only
/// inside this volume.
#[derive(Debug, Clone)]
pub struct Terrain {
    /// Max bounduary box (cell cannot be placed outside of this volume)
    volume: Volume,
    /// Maps that for each valid position returns the type of cell at that position.
    cells: HashMap<Position, CellType>,
}

impl Serialize for Terrain {
    /// Serialize contain of Terrain
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.cells.len()))?;
        for cell in &self.cells {
            seq.serialize_element(&cell)?;
        }

        seq.end()
    }
}

impl Terrain {
    /// Create an empty terrain with provided volume.
    ///
    /// * `volume`: Max bounduary box (cell cannot be placed outside of this volume)
    pub fn new(volume: &Volume) -> Self {
        Self {
            volume: *volume,
            cells: HashMap::new(),
        }
    }

    /// Gets volume of this terrain. That is max bounduary box (cell cannot be placed outside of this volume).
    pub fn volume(&self) -> Volume {
        self.volume
    }

    /// Get the min bounduary box that contains all cells of this terrain
    pub fn bounduary_box(&self) -> Option<Volume> {
        if self.cells.is_empty() {
            // No cell, no fun!
            None
        } else {
            // Compute the lower and upper corner
            let mut lower = (i32::MAX, i32::MAX, i32::MAX);
            let mut upper = (i32::MIN, i32::MIN, i32::MIN);

            for (p, _) in &self.cells {
                if p.x() < lower.0 {
                    lower.0 = p.x();
                }
                if p.y() < lower.1 {
                    lower.1 = p.y();
                }
                if p.z() < lower.2 {
                    lower.2 = p.z();
                }
                if p.x() > upper.0 {
                    upper.0 = p.x();
                }
                if p.y() > upper.1 {
                    upper.1 = p.y();
                }
                if p.z() > upper.2 {
                    upper.2 = p.z();
                }
            }
            let lower_position = Position::new(lower.0, lower.1, lower.2);
            let upper_position = Position::new(upper.0, upper.1, upper.2);
            Some(
                Volume::new(&lower_position, &upper_position)
                    .expect("Bounduary shall be a valid volume"),
            )
        }
    }

    /// Get cells type at selected position
    ///
    /// Returns `None` if position is not valid (outside the volume) or
    /// position is empty
    pub fn get_cell_at(&self, position: &Position) -> Option<CellType> {
        if !self.volume.is_inside(position) {
            warn!("Position {:?} is outside of terrain", position);
            return None;
        }
        self.cells.get(position).copied()
    }

    /// Set the cell type at selected position
    ///
    /// The cell is update only if the position is inside the volume and return `true`, otherwise returns `false`
    pub fn set_cell_at(&mut self, position: &Position, cell_type: CellType) -> bool {
        if self.volume.is_inside(&position) {
            self.cells.insert(*position, cell_type);
            true
        } else {
            warn!(
                "Position {:?} is not valid for Terrain {:?}",
                position, self.volume
            );
            false
        }
    }

    /// Remove a cell at selected position
    ///
    /// If cell existed return true
    pub fn remove_cell_at(&mut self, position: &Position) -> bool {
        if self.volume.is_inside(position) {
            self.cells.remove(position).is_some()
        } else {
            warn!(
                "Position {:?} is not valid for terrain {:?}",
                position, self.volume
            );
            false
        }
    }
}

impl<'a> IntoIterator for &'a Terrain {
    /// Type of elements being iterated over
    type Item = (&'a Position, &'a CellType);
    /// Type of iterator being turned into
    type IntoIter = Iter<'a, Position, CellType>;

    /// Returns an iterator that iterate along all position inside
    /// this volume
    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utilities::random_generator::*;
    use crate::{geometry::vector::Scalar, test_utilities::constants::*};

    #[test]
    /// Check constructor
    fn new_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_SMALL_TEST {
            let vol = random_volume(1, 15);
            let terrain = Terrain::new(&vol);

            assert_eq!(terrain.volume(), vol);
            assert!(terrain.bounduary_box().is_none());

            for p in &vol {
                assert!(terrain.get_cell_at(&p).is_none());
            }
        }
    }
    #[test]
    fn get_outside_test() {
        const SIZE: Scalar = 5;
        const DELTA: Scalar = 5;

        let blc = random_vector(-SIZE, SIZE);
        let diagonal = random_vector(1, SIZE);
        let trc = blc + diagonal;
        let vol = Volume::new(&blc, &trc).unwrap();
        let blc = vol.bottom_left_corner();
        let mut terrain = Terrain::new(&vol);

        for p in &vol {
            assert!(terrain.set_cell_at(&p, CellType::Fill(CellMaterial::Water)));
        }

        // Create all points in an area around volume with a panning of DELTA
        for x in blc.x() - DELTA..blc.x() + DELTA {
            for y in blc.y() - DELTA..blc.y() + DELTA {
                for z in blc.z() - DELTA..blc.z() + DELTA {
                    let point = Position::new(x, y, z);

                    assert_eq!(
                        vol.is_inside(&point),
                        terrain.get_cell_at(&point).is_some(),
                        "Point {:?} - Volume {:?}",
                        point,
                        vol
                    );
                }
            }
        }
    }

    #[test]
    /// Set and gets random cell inside and outside a terrain and check if it is consistent
    fn set_get_cells_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let vol = random_volume(1, 50);
            let mut terrain = Terrain::new(&vol);

            // Generate a random position
            let pos = random_vector(-100, 100);

            // Try to set a cell
            let cell = random_cell();
            let sr = terrain.set_cell_at(&pos, cell);

            // Set will fail if position is outside
            assert_eq!(sr, vol.is_inside(&pos));

            if sr {
                // If set was ok, get should return the right thing
                assert_eq!(terrain.get_cell_at(&pos).unwrap(), cell);
                assert!(vol.is_inside(&pos));
            } else {
                // Should return None
                assert!(terrain.get_cell_at(&pos).is_none());
                assert!(!vol.is_inside(&pos));
            }
        }
    }

    #[test]
    /// Check if remove_cell_at cell works
    fn remove_cell_at_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let vol = random_volume(1, 6);
            let mut terrain = Terrain::new(&vol);
            let mut n = 0i32;

            // Fill the volume with Block cell
            for pos in &vol {
                assert!(terrain.set_cell_at(&pos, random_cell()));
                n += 1;
                assert_eq!(terrain.into_iter().count(), n as usize);
            }

            // Remove all cells
            n = vol.volume();
            for pos in &vol {
                // Remove cell
                assert!(terrain.remove_cell_at(&pos));

                // Cell should not be exist anymore
                assert!(terrain.get_cell_at(&pos).is_none());
                n -= 1;
                assert_eq!(terrain.into_iter().count(), n as usize);
            }

            // Terrain is empty
            assert_eq!(terrain.into_iter().count(), 0);

            // Removing cell outside and inside (but already removed)
            for x in vol.bottom_left_corner().x() - 2..vol.bottom_left_corner().x() + 2 {
                for y in vol.bottom_left_corner().y() - 2..vol.bottom_left_corner().y() + 2 {
                    for z in vol.bottom_left_corner().z() - 2..vol.bottom_left_corner().z() + 2 {
                        let pos = Position::new(x, y, z);

                        assert_eq!(terrain.remove_cell_at(&pos), false);
                    }
                }
            }
        }
    }

    #[test]
    /// Check if serialization + deserialization of a terrain is consistent
    fn serialize_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_SMALL_TEST {
            let vol = random_volume(1, 10);
            let (t, cells) = random_terrain(&vol);
            let serialized_t = serde_json::to_string(&t).ok().expect("Cannot serialize");
            assert_eq!(cells.len(), t.into_iter().count());
            // For each cell we need at least 24 chars.
            assert!(
                serialized_t.len() > cells.len() * 24,
                "{} > {}",
                serialized_t.len(),
                cells.len() * 24
            );
        }
    }
}
