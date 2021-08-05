use std::marker::PhantomData;
use std::ops::{ Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg };

use crate::misc::generics_int::{ GenericsInt };

#[derive(Copy, Clone, PartialEq)]
pub struct ModInt<G> {
    value: u64,
    phantom: PhantomData<G>
}

impl<G: GenericsInt<Output = u64>> ModInt<G> {
    pub fn new() -> Self {
        ModInt { value: 0, phantom: PhantomData }
    }

    pub fn pow(self, mut p: u64) -> Self {
        let mut ret = 1;
        let mut a = self.value;

        while p > 0 {
            if (p & 1) != 0 {
                ret *= a;
                ret %= G::value();
            }

            a *= a;
            a %= G::value();

            p >>= 1;
        }

        Self { value: ret, phantom: PhantomData }
    }

    pub fn inv(self) -> Self {
        self.pow(G::value() - 2)
    }
}

impl<G> std::fmt::Display for ModInt<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<G: GenericsInt<Output = u64>> std::fmt::Debug for ModInt<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (mod {})", self.value, G::value())
    }
}

macro_rules! modint_from_int {
    ( $($t:ty),* ) => {
        $(
            impl<G: GenericsInt<Output = u64>> From<$t> for ModInt<G> {
                fn from(from: $t) -> Self {
                    let mut value = ((from % G::value() as $t) + G::value() as $t) as u64;
                    if value >= G::value() {
                        value -= G::value();
                    }

                    ModInt { value: value, phantom: PhantomData }
                }
            }
        )*
    }
}

modint_from_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl<G> From<ModInt<G>> for u64 {
    fn from(from: ModInt<G>) -> Self {
        from.value
    }
}

impl<G: GenericsInt<Output = u64>> Add for ModInt<G> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { value: (u64::from(self) + u64::from(other)) % G::value(), phantom: PhantomData }
    }
}

impl<G: GenericsInt<Output = u64> + Copy> AddAssign for ModInt<G> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<G: GenericsInt<Output = u64>> Sub for ModInt<G> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { value: (u64::from(self) + (G::value() - u64::from(other))) % G::value(), phantom: PhantomData }
    }
}

impl<G: GenericsInt<Output = u64> + Copy> SubAssign for ModInt<G> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<G: GenericsInt<Output = u64>> Mul for ModInt<G> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { value: (u64::from(self) * u64::from(other)) % G::value(), phantom: PhantomData }
    }
}

impl<G: GenericsInt<Output = u64> + Copy> MulAssign for ModInt<G> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<G: GenericsInt<Output = u64>> Div for ModInt<G> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self * other.inv()
    }
}

impl<G: GenericsInt<Output = u64> + Copy> DivAssign for ModInt<G> {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl<G: GenericsInt<Output = u64>> Neg for ModInt<G> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: G::value() - u64::from(self), phantom: PhantomData }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::generics_int;

    generics_int!(G, 1000000007);
    type Mint = ModInt<G>;

    #[test]
    fn test() {
    }
}