//use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;

/// Type used to store Vector components
pub type Scalar = i32;

/// Represent a generic vector of integers
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector(Scalar, Scalar, Scalar);

impl Vector {
    /// Create a new vector with given components
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Self(x, y, z)
    }

    /// Returns the X component
    pub fn x(&self) -> Scalar {
        self.0
    }

    /// Returns the Y component
    pub fn y(&self) -> Scalar {
        self.1
    }

    /// Returns the Z component
    pub fn z(&self) -> Scalar {
        self.2
    }

    /// Returns true if all components are equal or greater than 0
    pub fn is_positive(&self) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.2 >= 0
    }

    /// Returns a vector with components equal to provided one, but with absolute values
    pub fn abs(&self) -> Self {
        Self::new(self.0.abs(), self.1.abs(), self.2.abs())
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    /// Add vector components each other
    fn add(self, other: Vector) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    /// Subtract vector components each other
    fn sub(self, other: Vector) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utilities::constants::*;
    use crate::test_utilities::random_generator::*;

    #[test]
    /// Check if is_positive returns only for vector with only
    /// positive components
    fn is_positive_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_BIG_TEST {
            // Random number
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);
            // Vector
            let v = Vector::new(x0, y0, z0);

            assert_eq!(v.is_positive(), x0 >= 0 && y0 >= 0 && z0 >= 0);
            assert_eq!(!v.is_positive(), x0 < 0 || y0 < 0 || z0 < 0);
        }
    }

    #[test]
    /// Check if add compute right values
    fn add_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(0 as Scalar, 100 as Scalar);
            let y0 = random_number(0 as Scalar, 100 as Scalar);
            let z0 = random_number(0 as Scalar, 100 as Scalar);
            // Vector 1 components
            let x1 = random_number(0 as Scalar, 100 as Scalar);
            let y1 = random_number(0 as Scalar, 100 as Scalar);
            let z1 = random_number(0 as Scalar, 100 as Scalar);

            let v0 = Vector(x0, y0, z0);
            let v1 = Vector(x1, y1, z1);

            let v_sum = v0 + v1;

            assert_eq!(v_sum.0, x0 + x1);
            assert_eq!(v_sum.1, y0 + y1);
            assert_eq!(v_sum.2, z0 + z1);
        }
    }

    #[test]
    /// Check if sub compute right values
    fn sub_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(0 as Scalar, 100 as Scalar);
            let y0 = random_number(0 as Scalar, 100 as Scalar);
            let z0 = random_number(0 as Scalar, 100 as Scalar);
            // Vector 1 components
            let x1 = random_number(0 as Scalar, 100 as Scalar);
            let y1 = random_number(0 as Scalar, 100 as Scalar);
            let z1 = random_number(0 as Scalar, 100 as Scalar);

            let v0 = Vector(x0, y0, z0);
            let v1 = Vector(x1, y1, z1);

            let v_sum = v0 - v1;

            assert_eq!(v_sum.0, x0 - x1);
            assert_eq!(v_sum.1, y0 - y1);
            assert_eq!(v_sum.2, z0 - z1);
        }
    }

    #[test]
    /// Check if getters method work
    fn getter_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(0 as Scalar, 100 as Scalar);
            let y0 = random_number(0 as Scalar, 100 as Scalar);
            let z0 = random_number(0 as Scalar, 100 as Scalar);

            let v0 = Vector(x0, y0, z0);

            assert_eq!(v0.0, v0.x());
            assert_eq!(v0.1, v0.y());
            assert_eq!(v0.2, v0.z());
        }
    }

    #[test]
    /// Check if constructor does save provided parameters in right way
    fn constructor_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(0 as Scalar, 100 as Scalar);
            let y0 = random_number(0 as Scalar, 100 as Scalar);
            let z0 = random_number(0 as Scalar, 100 as Scalar);

            let v0 = Vector::new(x0, y0, z0);

            assert_eq!(v0.0, x0);
            assert_eq!(v0.1, y0);
            assert_eq!(v0.2, z0);
        }
    }

    #[test]
    /// Check if abs is computing a vector with all components with absolute value
    fn abs_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            let v = random_vector(-100, 100);
            let av = v.abs();

            assert_eq!(v.0.abs(), av.0);
            assert_eq!(v.1.abs(), av.1);
            assert_eq!(v.2.abs(), av.2);
        }
    }
}
