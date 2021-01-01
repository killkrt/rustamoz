#![cfg(test)]
#![allow(dead_code)]

/// Compare two floating points with some tollerance
pub fn nearly_equal(a: f32, b: f32) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < std::f32::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (std::f32::EPSILON * std::f32::MIN_POSITIVE)
    } else {
        // Use relative error.
        (diff / f32::min(abs_a + abs_b, std::f32::MAX)) < std::f32::EPSILON
    }
}

pub fn check_for_duplicate<T>(list: &mut Vec<T>) -> bool
where
    T: Ord,
{
    // Remove all duplicates
    let len = list.len();
    list.sort();
    list.dedup();
    // No duplicates shall be removed after removing duplicates
    list.len() == len
}
