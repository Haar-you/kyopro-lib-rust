//! Montgomery乗算
//!
//! # References
//! - <https://ja.wikipedia.org/wiki/%E3%83%A2%E3%83%B3%E3%82%B4%E3%83%A1%E3%83%AA%E4%B9%97%E7%AE%97>
use crate::impl_ops;
use crate::num::ff::*;

/// [`Montgomery`]を生成するための構造体。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MontgomeryBuilder {
    modulo: u32,
    r2: u64,
    m: u64,
}

const B: u32 = 32;
const R: u64 = 1 << B;
const MASK: u64 = R - 1;

impl MontgomeryBuilder {
    /// `modulo`を法とする[`MontgomeryBuilder`]を生成する。
    pub fn new(modulo: u32) -> Self {
        assert!(modulo % 2 != 0);
        assert!(modulo > 0);

        let r = R % modulo as u64;
        let r2 = r * r % modulo as u64;
        let m = {
            let mut ret = 0;
            let mut r = R;
            let mut i = 1;
            let mut t = 0;
            while r > 1 {
                if t % 2 == 0 {
                    t += modulo;
                    ret += i;
                }
                t >>= 1;
                r >>= 1;
                i <<= 1;
            }
            ret
        };

        Self { modulo, r2, m }
    }
}

fn reduce(value: u64, modulo: u64, m: u64) -> u64 {
    let mut ret = ((((value & MASK) * m) & MASK) * modulo + value) >> B;
    if ret >= modulo {
        ret -= modulo;
    }
    ret
}

impl FF for MontgomeryBuilder {
    type Element = Montgomery;
    fn from_u64(&self, mut value: u64) -> Self::Element {
        if value >= self.modulo as u64 {
            value %= self.modulo as u64;
        }

        let value = reduce(value * self.r2, self.modulo as u64, self.m);
        Montgomery::__new(value, self.modulo as u64, self.r2, self.m)
    }

    fn from_i64(&self, mut value: i64) -> Self::Element {
        value %= self.modulo as i64;
        if value < 0 {
            value += self.modulo as i64;
        }

        let value = reduce(value as u64 * self.r2, self.modulo as u64, self.m);
        Montgomery::__new(value, self.modulo as u64, self.r2, self.m)
    }
}

/// `modulo`を法として剰余をとる構造体。
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Montgomery {
    value: u64,
    modulo: u64,
    r2: u64,
    m: u64,
}

impl FFElem for Montgomery {
    #[inline]
    fn value(self) -> u32 {
        reduce(self.value, self.modulo, self.m) as u32
    }

    #[inline]
    fn modulo(self) -> u32 {
        self.modulo as u32
    }

    fn pow(self, mut p: u64) -> Self {
        let mut value = reduce(self.r2, self.modulo, self.m);
        let mut a = self.value;

        while p > 0 {
            if (p & 1) != 0 {
                value = reduce(value * a, self.modulo, self.m);
            }
            a = reduce(a * a, self.modulo, self.m);
            p >>= 1;
        }

        Self { value, ..self }
    }
}

impl Montgomery {
    fn __new(value: u64, modulo: u64, r2: u64, m: u64) -> Self {
        Self {
            value,
            modulo,
            r2,
            m,
        }
    }
}

impl_ops!(Add for Montgomery, |mut x: Self, y| {
    x += y;
    x
});

impl_ops!(Sub for Montgomery, |mut x: Self, y| {
    x -= y;
    x
});

impl_ops!(Mul for Montgomery, |mut x: Self, y| {
    x *= y;
    x
});

impl_ops!(Div for Montgomery, |mut x: Self, y| {
    x /= y;
    x
});

impl_ops!(AddAssign for Montgomery, |x: &mut Self, y: Self| {
    x.value += y.value;
    if x.value >= x.modulo {
        x.value -= x.modulo;
    }
});

impl_ops!(SubAssign for Montgomery, |x: &mut Self, y: Self| {
    if x.value < y.value {
        x.value += x.modulo;
    }
    x.value -= y.value;
});

impl_ops!(MulAssign for Montgomery, |x: &mut Self, y: Self| x.value =
    reduce(x.value * y.value, x.modulo, x.m));

impl_ops!(DivAssign for Montgomery, |x: &mut Self, y: Self| *x *= y.inv());

impl_ops!(Neg for Montgomery, |mut x: Self| {
    if x.value != 0 {
        x.value = x.modulo - x.value;
    }
    x
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iter::collect::CollectVec;
    use crate::num::{const_modint::*, modint::*};
    use crate::timer;
    use rand::Rng;

    #[derive(Clone, Copy, Debug)]
    enum Ops {
        Add(u64),
        Sub(u64),
        Mul(u64),
        Div(u64),
        Neg,
    }

    #[test]
    fn test() {
        const MOD: u32 = 998244353;

        let mut rng = rand::thread_rng();

        let constmodint = ConstModIntBuilder::<MOD>;
        let modint = ModIntBuilder::new(MOD);
        let montgomery = MontgomeryBuilder::new(MOD);

        let mut ans = constmodint.from_u64(1);
        let mut ans2 = modint.from_u64(1);
        let mut res = montgomery.from_u64(1);

        let ops = (0..1000000)
            .map(|_| {
                let x = rng.gen_range(1..MOD) as u64;

                let op = rng.gen_range(0..5);
                match op {
                    0 => Ops::Add(x),
                    1 => Ops::Sub(x),
                    2 => Ops::Mul(x),
                    3 => Ops::Div(x),
                    4 => Ops::Neg,
                    _ => unreachable!(),
                }
            })
            .collect_vec();

        timer! {{
            for &op in &ops {
                match op {
                    Ops::Add(x) => ans += constmodint.from_u64(x),
                    Ops::Sub(x) => ans -= constmodint.from_u64(x),
                    Ops::Mul(x) => ans *= constmodint.from_u64(x),
                    Ops::Div(x) => ans /= constmodint.from_u64(x),
                    Ops::Neg => ans = -ans
                }
            }
        }};

        timer! {{
            for &op in &ops {
                match op {
                    Ops::Add(x) => ans2 += modint.from_u64(x),
                    Ops::Sub(x) => ans2 -= modint.from_u64(x),
                    Ops::Mul(x) => ans2 *= modint.from_u64(x),
                    Ops::Div(x) => ans2 /= modint.from_u64(x),
                    Ops::Neg => ans2 = -ans2
                }
            }
        }};

        timer! {{
            for &op in &ops {
                match op {
                    Ops::Add(x) => res += montgomery.from_u64(x),
                    Ops::Sub(x) => res -= montgomery.from_u64(x),
                    Ops::Mul(x) => res *= montgomery.from_u64(x),
                    Ops::Div(x) => res /= montgomery.from_u64(x),
                    Ops::Neg => res = -res
                }
            }
        }};

        dbg!(ans.value());
        dbg!(ans2.value());
        dbg!(res.value());

        assert_eq!(ans.value(), ans2.value());
        assert_eq!(ans.value(), res.value());
    }
}
