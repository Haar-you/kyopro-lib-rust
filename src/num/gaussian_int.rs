//! ガウス整数
use crate::impl_ops;

/// ガウス整数 $a + bi (a, b \in \mathbb{Z})$
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq, Hash)]
pub struct GaussianInt {
    /// 実部
    pub re: i64,
    /// 虚部
    pub im: i64,
}

impl GaussianInt {
    /// ガウス整数$a + bi$を返す。
    pub fn new(a: i64, b: i64) -> Self {
        Self { re: a, im: b }
    }

    /// `self`のノルム$N(\mathtt{self})$を返す。
    ///
    /// ただし、$N(a + bi) = a^2 + b^2$
    pub fn norm(self) -> u64 {
        (self.re * self.re + self.im * self.im) as u64
    }

    /// $\mathtt{self} = q \times \mathtt{b} + r (N(b) > N(r))$となる$q$と$r$を返す。
    pub fn div_rem(self, b: Self) -> (Self, Self) {
        let a = self;
        assert!(b.re != 0 || b.im != 0);

        let deno = b.re * b.re + b.im * b.im;
        let d_re = (a.re * b.re + a.im * b.im) / deno;
        let d_im = (a.im * b.re - a.re * b.im) / deno;

        let (q, _) = (d_re - 1..=d_re + 1)
            .flat_map(|re| {
                (d_im - 1..=d_im + 1).map(move |im| {
                    let q = Self { re, im };
                    let d = (a - q * b).norm();
                    (q, d)
                })
            })
            .min_by_key(|(_, d)| *d)
            .unwrap();

        (q, self - q * b)
    }

    /// `self`が$0 + 0i$ならば`true`を返す。
    pub fn is_zero(self) -> bool {
        self.re == 0 && self.im == 0
    }

    /// `self`と`b`の最大公約数を返す。
    pub fn gcd(self, b: Self) -> Self {
        if b.is_zero() {
            self
        } else {
            b.gcd(self % b)
        }
    }
}

impl_ops!(Add for GaussianInt, |a: Self, b: Self| Self {
    re: a.re + b.re,
    im: a.im + b.im,
});
impl_ops!(Sub for GaussianInt, |a: Self, b: Self| Self {
    re: a.re - b.re,
    im: a.im - b.im,
});
impl_ops!(Mul for GaussianInt, |a: Self, b: Self| Self {
    re: a.re * b.re - a.im * b.im,
    im: a.re * b.im + a.im * b.re
});

impl_ops!(AddAssign for GaussianInt, |a: &mut Self, b: Self| *a = *a + b);
impl_ops!(SubAssign for GaussianInt, |a: &mut Self, b: Self| *a = *a - b);
impl_ops!(MulAssign for GaussianInt, |a: &mut Self, b: Self| *a = *a * b);

impl_ops!(
    /// $\mathtt{self} = q \times \mathtt{b} + r (N(b) > N(r))$となる$q$を返す。
    Div for GaussianInt, |a: Self, b: Self| a.div_rem(b).0);
impl_ops!(
    /// $\mathtt{self} = q \times \mathtt{b} + r (N(b) > N(r))$となる$r$を返す。
    Rem for GaussianInt, |a: Self, b: Self| a.div_rem(b).1);

impl_ops!(DivAssign for GaussianInt, |a: &mut Self, b: Self| *a = *a / b);
impl_ops!(RemAssign for GaussianInt, |a: &mut Self, b: Self| *a = *a % b);
