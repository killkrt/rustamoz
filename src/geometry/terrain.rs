use std::collections::HashMap;

use log::warn;

use super::{vector::Position, volume::Volume};

/// Possible types of cell materials
#[derive(Debug, Copy, Clone)]
pub enum CellMaterial {
    Water,
    Ground,
}

/// Possible types of cell
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug)]
pub struct Terrain {
    /// Bounding box
    volume: Volume,
    /// Maps that for each valid position returns the type of cell at that position.
    cells: HashMap<Position, CellType>,
}

impl Terrain {
    /// Create an empty terrain with provided volume.
    ///
    /// The terrain doesn't contain any cells.
    pub fn new(volume: &Volume) -> Self {
        Self {
            volume: *volume,
            cells: HashMap::new(),
        }
    }

    /// Get cells type at selected position
    ///
    /// Returns `None` if position is not valid (outside the volume) or
    /// position is empty
    pub fn get_at(&self, position: &Position) -> Option<CellType> {
        if !self.volume.is_inside(position) {
            warn!("Position {:?} is outside of terrain", position);
            return None;
        }
        self.cells.get(position).copied()
    }

    /// Gets volume of this terrain.
    pub fn volume(&self) -> Volume {
        self.volume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utilities::constants::*;
    use crate::test_utilities::random_generator::*;

    #[test]
    /// Check constructor
    fn new_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_SMALL_TEST {
            let vol = random_volume(1, 15);
            let terrain = Terrain::new(&vol);

            assert_eq!(terrain.volume(), vol);

            for p in &vol {
                assert!(terrain.get_at(&p).is_none());
            }
        }
    }
    #[test]
    fn get_outside_test() {}
}
