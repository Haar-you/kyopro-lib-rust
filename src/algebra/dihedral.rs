//! 二面体群
//!
//! # Problems
//! - <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0390>

pub use crate::algebra::traits::*;

/// 二面体群の元
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DihedralValue {
    /// 回転
    R(usize, usize),
    /// 鏡映
    S(usize, usize),
}

impl DihedralValue {
    fn get_k(self) -> usize {
        match self {
            Self::R(_, k) => k,
            Self::S(_, k) => k,
        }
    }

    fn op(a: Self, b: Self) -> Self {
        let k = a.get_k();
        assert_eq!(k, b.get_k());

        match (a, b) {
            (Self::R(x, _), Self::R(y, _)) => Self::R((x + y) % k, k),
            (Self::R(x, _), Self::S(y, _)) => Self::S((k + y - x) % k, k),
            (Self::S(x, _), Self::R(y, _)) => Self::S((x + y) % k, k),
            (Self::S(x, _), Self::S(y, _)) => Self::R((k + y - x) % k, k),
        }
    }
}

/// 二面体群
#[derive(Clone, Copy, Debug, Default)]
pub enum Dihedral {
    #[default]
    Id,
    Value(DihedralValue),
}

impl Dihedral {
    pub fn eval(self, k: usize) -> DihedralValue {
        match self {
            Self::Id => DihedralValue::R(0, k),
            Self::Value(a) => a,
        }
    }

    pub fn is_identity(self) -> bool {
        match self {
            Self::Id => true,
            Self::Value(a) => matches!(a, DihedralValue::R(0, _)),
        }
    }
}

impl PartialEq for Dihedral {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a == b,
            _ => self.is_identity() && other.is_identity(),
        }
    }
}

impl Set for Dihedral {}

impl BinaryOp for Dihedral {
    fn op(self, b: Self) -> Self {
        match (self, b) {
            (Self::Value(a), Self::Value(b)) => Self::Value(DihedralValue::op(a, b)),
            (a @ Self::Value(_), _) => a,
            (_, b @ Self::Value(_)) => b,
            _ => Self::Id,
        }
    }
}

impl Identity for Dihedral {
    fn id() -> Self {
        Self::Id
    }
}

impl Inverse for Dihedral {
    fn inv(self) -> Self {
        match self {
            Self::Id => self,
            Self::Value(a) => Self::Value(match a {
                DihedralValue::R(x, k) => DihedralValue::R(if x == 0 { 0 } else { k - 1 }, k),
                DihedralValue::S(_, _) => a,
            }),
        }
    }
}

impl Associative for Dihedral {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let k = 3;

        let a = (0..k)
            .map(|i| DihedralValue::R(i, k))
            .chain((0..k).map(|i| DihedralValue::S(i, k)))
            .collect::<Vec<_>>();

        for l in &a {
            for r in &a {
                let l = Dihedral::Value(*l);
                let r = Dihedral::Value(*r);

                let x = r.op(l);

                println!("{:?} {:?} = {:?}", l, r, x)
            }
        }
    }
}
