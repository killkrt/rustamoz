use serde::Serialize;
use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Sub;

/// Type used to store Vector components
pub type Scalar = i32;

/// Represent a generic vector of integers
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Vector(Scalar, Scalar, Scalar);

/// A position is a Vector
pub type Position = Vector;

/// A distance is a Vector
pub type Distance = Vector;

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

    /// Returns true if all components are equal to 0
    pub fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0 && self.2 == 0
    }

    /// Returns a vector with components equal to provided one, but with absolute values
    pub fn abs(&self) -> Self {
        Self::new(self.0.abs(), self.1.abs(), self.2.abs())
    }

    /// Returns a vector with all components set to zero.
    pub fn zero() -> Self {
        Self::new(0, 0, 0)
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

impl PartialOrd for Vector {
    /// Comparison criteria:
    /// * Equal - all components are equal
    /// * Less - all components are less than respectively others
    /// * Great - all components are great than respectively others
    ///
    /// Returns `Option::None` if none of the above apply
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            // All components are equal than other
            Some(Ordering::Equal)
        } else {
            if self.0 < other.0 && self.1 < other.1 && self.2 < other.2 {
                // All components are less than other
                Some(Ordering::Less)
            } else {
                if self.0 > other.0 && self.1 > other.1 && self.2 > other.2 {
                    // All components are greater than other
                    Some(Ordering::Greater)
                } else {
                    // Not clear comparison possible
                    None
                }
            }
        }
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
    /// Check if is_zero returns only for vector with only
    /// 0 components
    fn is_zero_test() {
        assert!(Vector::zero().is_zero());

        for _ in 0..NUMBER_OF_LOOPS_FOR_BIG_TEST {
            // Random number
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);
            // Vector
            let v = Vector::new(x0, y0, z0);

            assert_eq!(v.is_zero(), x0 == 0 && y0 == 0 && z0 == 0);
            assert_eq!(!v.is_zero(), x0 != 0 || y0 != 0 || z0 != 0);
        }
    }

    #[test]
    /// Check if add compute right values
    fn add_test() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);
            // Vector 1 components
            let x1 = random_number(-100 as Scalar, 100 as Scalar);
            let y1 = random_number(-100 as Scalar, 100 as Scalar);
            let z1 = random_number(-100 as Scalar, 100 as Scalar);

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
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);
            // Vector 1 components
            let x1 = random_number(-100 as Scalar, 100 as Scalar);
            let y1 = random_number(-100 as Scalar, 100 as Scalar);
            let z1 = random_number(-100 as Scalar, 100 as Scalar);

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
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);

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
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);

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

    #[test]
    /// Check if v + 0 = v
    fn add_zero() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);

            let v0 = Vector(x0, y0, z0);

            let v_sum = v0 + Vector::zero();

            assert_eq!(v_sum.0, x0);
            assert_eq!(v_sum.1, y0);
            assert_eq!(v_sum.2, z0);
        }
    }

    #[test]
    /// Check if v - 0 = v
    fn sub_zero() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);

            let v0 = Vector(x0, y0, z0);

            let v_sum = v0 - Vector::zero();

            assert_eq!(v_sum.0, x0);
            assert_eq!(v_sum.1, y0);
            assert_eq!(v_sum.2, z0);
        }
    }

    #[test]
    /// Check if v - v = 0
    fn sum_to_zero() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);
            let v0 = Vector(x0, y0, z0);

            // Vector 1 as -v0
            let v1 = Vector(-x0, -y0, -z0);
            let v0_sum = v0 - v0;
            let v1_sum = v0 + v1;

            assert_eq!(v0_sum.0, 0);
            assert_eq!(v0_sum.1, 0);
            assert_eq!(v0_sum.2, 0);

            assert_eq!(v1_sum.0, 0);
            assert_eq!(v1_sum.1, 0);
            assert_eq!(v1_sum.2, 0);

            assert_eq!(v0_sum, Vector::zero());
            assert_eq!(v1_sum, Vector::zero());
        }
    }

    #[test]
    /// Subtract a random vector to vector 0
    fn subtract_to_zero() {
        for _ in 0..NUMBER_OF_LOOPS_FOR_NORMAL_TEST {
            // Vector 0 components
            let x0 = random_number(-100 as Scalar, 100 as Scalar);
            let y0 = random_number(-100 as Scalar, 100 as Scalar);
            let z0 = random_number(-100 as Scalar, 100 as Scalar);
            let v0 = Vector(x0, y0, z0);

            // Vector 1 as -v0
            let sub = Vector::zero() - v0;

            assert_eq!(sub.0, -x0);
            assert_eq!(sub.1, -y0);
            assert_eq!(sub.2, -z0);
        }
    }

    #[test]
    /// Compares different vectors
    fn comparison_test() {
        let mut eq_cnt = 0;
        let mut less_cnt = 0;
        let mut great_cnt = 0;
        let mut none_cnt = 0;

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
            let v0 = Vector(x0, y0, z0);
            // Vector 1 components
            let x1 = components[0];
            let y1 = components[1];
            let z1 = components[2];
            let v1 = Vector(x1, y1, z1);

            if x0 == x1 && y0 == y1 && z0 == z1 {
                assert_eq!(v0.partial_cmp(&v1), Some(Ordering::Equal));
                eq_cnt += 1;
            } else {
                if x0 < x1 && y0 < y1 && z0 < z1 {
                    assert_eq!(v0.partial_cmp(&v1), Some(Ordering::Less));
                    less_cnt += 1;
                } else {
                    if x0 > x1 && y0 > y1 && z0 > z1 {
                        assert_eq!(v0.partial_cmp(&v1), Some(Ordering::Greater));
                        great_cnt += 1;
                    } else {
                        assert_eq!(v0.partial_cmp(&v1), None);
                        none_cnt += 1;
                    }
                }
            }
        }
        assert_eq!(eq_cnt, 1);
        assert_eq!(less_cnt, 2);
        assert_eq!(great_cnt, 2);
        assert_eq!(none_cnt, 4);
    }
}
