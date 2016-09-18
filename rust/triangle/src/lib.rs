extern crate num;

use num::traits::identities::Zero;
use std::cmp::PartialOrd;

pub struct Triangle<T: Zero + PartialOrd + Copy>([T; 3]);

impl<T: Zero + PartialOrd + Copy> Triangle<T> {
    pub fn build(lengths: [T; 3]) -> Result<Triangle<T>, &'static str> {
        if lengths[0] <= T::zero() || lengths[1] <= T::zero() || lengths[2] <= T::zero() {
            Err("Zero sized sides are illegal")
        } else if !(lengths[0] + lengths[1] > lengths[2] && lengths[1] + lengths[2] > lengths[0] &&
                    lengths[2] + lengths[0] > lengths[1]) {
            Err("Triangle inequality does not hold")
        } else {
            Ok(Triangle(lengths))
        }
    }

    pub fn is_equilateral(&self) -> bool {
        // all three sides equal
        self.0[0] == self.0[1] && self.0[1] == self.0[2]
    }
    pub fn is_isosceles(&self) -> bool {
        // two sides are equal, but not all three
        !self.is_equilateral() &&
        (self.0[0] == self.0[1] || self.0[1] == self.0[2] || self.0[2] == self.0[0])
    }
    pub fn is_scalene(&self) -> bool {
        // all sides differently, no two sides equal
        self.0[0] != self.0[1] && self.0[1] != self.0[2] && self.0[2] != self.0[0]
    }
}
