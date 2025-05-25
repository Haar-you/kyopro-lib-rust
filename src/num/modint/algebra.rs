//! [`ModInt`]の代数的構造

pub use crate::algebra::traits::*;
use crate::{impl_algebra, num::modint::*};

/// `mod m`上の加法
#[derive(Clone, Copy, Default, Debug)]
pub enum SumModM {
    #[default]
    Zero,
    Value(ModInt),
}
impl_algebra!(
    SumModM;
    op: |a: Self, b: Self| match (a, b) {
        (Self::Value(a), Self::Value(b)) => Self::Value(a + b),
        (Self::Value(_), _) => a,
        (_, Self::Value(_)) => b,
        _ => Self::Zero
    };
    id: Self::Zero;
    inv: |a: Self| match a {
        Self::Value(a) => Self::Value(-a),
        _ => Self::Zero
    };
    assoc;
    commu;
);

/// `mod m`上の乗法
#[derive(Clone, Copy, Default, Debug)]
pub enum ProdModM {
    #[default]
    One,
    Value(ModInt),
}
impl_algebra!(
    ProdModM;
    op: |a: Self, b: Self| match (a, b) {
        (Self::Value(a), Self::Value(b)) => Self::Value(a * b),
        (Self::Value(_), _) => a,
        (_, Self::Value(_)) => b,
        _ => Self::One
    };
    id: Self::One;
    assoc;
    commu;
);
