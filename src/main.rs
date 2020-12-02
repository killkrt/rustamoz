mod geometry;

#[cfg(test)]
mod test_utilities;

fn main() {
    let v = geometry::vector::Vector::new(0, 1, 2);
    println!("{:?}", v);
}
