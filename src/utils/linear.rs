use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Linear<T> {
    pub a: T,
    pub b: T,
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Linear<T> {
    pub fn apply(&self, x: T) -> T {
        self.a * x + self.b
    }
}

impl<T: Sub<Output = T> + Mul<Output = T> + Copy> Linear<T> {
    pub fn mov_x(&self, dx: T) -> Self {
        Self {
            a: self.a,
            b: self.b - self.a * dx,
        }
    }
}
