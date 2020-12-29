#![cfg(test)]
#![allow(dead_code)]

use crate::geometry::{
    vector::{Scalar, Vector},
    volume::Volume,
};
use num::Num;

/// Generate a random cell coordinate to be used for test
pub fn random_number<T>(min: T, max: T) -> T
where
    T: Num + rand::distributions::uniform::SampleUniform,
{
    use rand::Rng;
    let mut rng = rand::thread_rng();
    // Generate a random number
    rng.gen_range(min, max)
}

/// Generate a vector with random coordinates and diagonal in range (min, max)
pub fn random_volume(min: Scalar, max: Scalar) -> Volume {
    let blc = random_vector(-100, 100);
    let diagonal = random_vector(min, max);
    let trc = blc + diagonal;

    Volume::new(&blc, &trc).expect("Invalid corners")
}

/// Generate a volume with random coordinates generate in the range of (min, max)
pub fn random_vector(min: Scalar, max: Scalar) -> Vector {
    let x = random_number(min as Scalar, max as Scalar);
    let y = random_number(min as Scalar, max as Scalar);
    let z = random_number(min as Scalar, max as Scalar);

    Vector::new(x, y, z)
}

/// Generate a random string with given len
pub fn random_string(len: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let text: String = std::iter::repeat(())
        .map(|()| thread_rng().sample(Alphanumeric))
        .take(len)
        .collect();

    text
}

// /// Generate a volume with position in range (min, max) and size in range (0, max_size)
// pub fn random_volume(min: Scalar, max: Scalar, max_size: Scalar) -> Volume {
//     let origin = Position::from(random_vector(min, max));
//     let other_corner = origin + Distance::from(random_vector(0, max_size));

//     Volume::new(&origin, &other_corner)
// }

// /// Generate a volume with origini in (0,0,0)
// pub fn random_volume_with_fixed_origin(
//     max_width: Scalar,
//     max_depth: Scalar,
//     max_height: Scalar,
// ) -> Volume {
//     let origin = Position::new(0, 0, 0);
//     let diag_x = random_number(1, max_width + 1);
//     let diag_y = random_number(1, max_depth + 1);
//     let diag_z = random_number(1, max_height + 1);
//     let other_corner = origin + Distance::new(diag_x, diag_y, diag_z);

//     Volume::new(&origin, &other_corner)
// }

// /// Generate a random cell type
// pub fn random_cell() -> CellType {
//     let cell_type_gen = random_number(0, 10) % 2;
//     let material_type_gen = random_number(0, 10) % 2;

//     let material_type = match material_type_gen {
//         0 => CellMaterial::Dust,
//         1 => CellMaterial::Water,
//         _ => panic!("Invalid type"),
//     };

//     match cell_type_gen {
//         0 => CellType::Flat(material_type),
//         1 => CellType::Fill(material_type),
//         _ => panic!("Invalid type"),
//     }
// }

// /// Generate a random terrain and list of cells generated
// pub fn random_terrain(volume: Volume) -> (Terrain, HashMap<Position, CellType>) {
//     let mut terrain = Terrain::new(volume);
//     let mut cells = HashMap::new();

//     for pos in &volume {
//         if random_number(0, 10) > 5 {
//             let cell = random_cell();
//             for z in 0..pos.vector().z() {
//                 let h = Position::new(pos.vector().x(), pos.vector().y(), z);
//                 terrain.set_cell_at(&h, cell);
//                 cells.insert(h, cell);
//             }
//         }
//     }

//     (terrain, cells)
// }
