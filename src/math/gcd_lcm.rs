//! 最大公約数・最小公倍数

use std::mem::swap;

/// 最大公約数・最小公倍数
pub trait GcdLcm {
    /// GCD,LCMの計算結果の型
    type Output;
    /// 最大公約数を求める。
    fn gcd(self, _: Self::Output) -> Self::Output;
    /// 最小公倍数を求める。
    fn lcm(self, _: Self::Output) -> Self::Output;
    /// 最大公約数と最小公倍数を求める。
    fn gcd_lcm(self, _: Self::Output) -> (Self::Output, Self::Output);
}

macro_rules! gcd_lcm_impl_all {
    ( $($t:ty),* ) => {
        $(
            impl GcdLcm for $t {
                type Output = $t;
                fn gcd(self, mut b: Self::Output) -> Self::Output {
                    let mut a = self;

                    if a < b {
                        swap(&mut a, &mut b);
                    }

                    if b == 0 {
                        return a;
                    }

                    b.gcd(a % b)
                }

                fn lcm(self, b: Self::Output) -> Self::Output {
                    self / self.gcd(b) * b
                }

                fn gcd_lcm(self, b: Self::Output) -> (Self::Output, Self::Output) {
                    let g = self.gcd(b);
                    (g, self / g * b)
                }
            }
        )*
    }
}

gcd_lcm_impl_all!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((12 as i32).gcd_lcm(8), (4, 24));
    }
}
