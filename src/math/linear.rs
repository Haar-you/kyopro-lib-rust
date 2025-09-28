//! $y = ax + b$の直線
use std::ops::{Add, Mul, Sub};

/// $y = ax + b$の直線を表す。
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Linear<T> {
    /// ‍直線の傾き
    pub a: T,
    /// 直線のy切片
    pub b: T,
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy> Linear<T> {
    /// `x`に値を代入した結果を返す。
    pub fn apply(&self, x: T) -> T {
        self.a * x + self.b
    }
}

impl<T: Sub<Output = T> + Mul<Output = T> + Copy> Linear<T> {
    /// x方向に`dx`だけ平行移動した直線を返す。
    pub fn mov_x(&self, dx: T) -> Self {
        Self {
            a: self.a,
            b: self.b - self.a * dx,
        }
    }
}
