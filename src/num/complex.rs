//! 複素数
use crate::impl_ops;

/// 複素数
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Complex {
    /// 実部
    pub re: f64,
    /// 虚部
    pub im: f64,
}

impl Complex {
    /// 直交座標系の複素数$a + bi$を返す。
    pub fn new(a: f64, b: f64) -> Self {
        Self { re: a, im: b }
    }

    /// 絶対値$|z| = \sqrt{a^2 + b^2}$を返す。
    pub fn abs(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// 偏角$\arg z \in (-\pi, \pi]$を返す。
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }

    /// 複素共役$\bar{z} = a - bi$を返す。
    pub fn conjugate(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// 極座標系の複素数$re^{ia}$を返す。
    pub fn polar(r: f64, a: f64) -> Self {
        Self {
            re: r * a.cos(),
            im: r * a.sin(),
        }
    }
}

impl_ops!(Add for Complex, |a: Self, b: Self| Self {
    re: a.re + b.re,
    im: a.im + b.im
});
impl_ops!(Sub for Complex, |a: Self, b: Self| Self {
    re: a.re - b.re,
    im: a.im - b.im,
});
impl_ops!(Mul for Complex, |a: Self, b: Self| Self {
    re: a.re * b.re - a.im * b.im,
    im: a.re * b.im + a.im * b.re
});
impl_ops!(Div for Complex, |a: Self, b: Self| Self {
    re: (a.re * b.re + a.im * b.im) / (b.re * b.re + b.im * b.im),
    im: (a.im * b.re - a.re * b.im) / (b.re * b.re + b.im * b.im)
});
impl_ops!(Neg for Complex, |a: Self| Self {
    re: -a.re,
    im: -a.im
});

impl_ops!(AddAssign for Complex, |a: &mut Self, b: Self| *a = *a + b);
impl_ops!(SubAssign for Complex, |a: &mut Self, b: Self| *a = *a - b);
impl_ops!(MulAssign for Complex, |a: &mut Self, b: Self| *a = *a * b);
impl_ops!(DivAssign for Complex, |a: &mut Self, b: Self| *a = *a / b);

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test() {
        assert_eq!(
            Complex::new(1.0, 1.0) + Complex::new(3.0, -2.5),
            Complex::new(4.0, -1.5)
        );

        assert_eq!(
            Complex::new(1.0, 1.0) - Complex::new(3.0, -2.5),
            Complex::new(-2.0, 3.5)
        );

        assert_eq!(
            Complex::new(1.0, 1.0) * Complex::new(3.0, -2.5),
            Complex::new(5.5, 0.5)
        );

        assert_eq!(
            Complex::polar(2.0, PI / 4.0) * Complex::polar(4.0, PI / 8.0),
            Complex::polar(8.0, PI * 3.0 / 8.0)
        );
    }
}
