//! 正の無限大、負の無限大をもつ数

use std::ops::{Add, Neg, Sub};

/// 正の無限大、負の無限大をもつ数
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NumInf<T> {
    /// 負の無限大
    NegInf,
    /// 有限の値
    Value(T),
    /// 正の無限大
    Inf,
}

impl<T: Copy> NumInf<T> {
    /// `self`が`Value(T)`かを判定する。
    pub fn is_value(self) -> bool {
        matches!(self, Self::Value(_))
    }

    /// `self`が`Inf`かを判定する。
    pub fn is_inf(self) -> bool {
        matches!(self, Self::Inf)
    }

    /// `self`が`NegInf`かを判定する。
    pub fn is_neg_inf(self) -> bool {
        matches!(self, Self::NegInf)
    }

    /// `self`が`Value`ならばその中身を取り出す。
    ///
    /// # Panics
    ///
    /// `self`が`Inf`か`NegInf`のときパニックする。
    pub fn unwrap(self) -> T {
        match self {
            Self::Value(x) => x,
            Self::Inf => panic!("called `NumInf::unwrap()` on a `Inf` value"),
            Self::NegInf => panic!("called `NumInf::unwrap()` on a `NegInf` value"),
        }
    }
}

impl<T: Add<Output = T>> Add for NumInf<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Self::Value(x) => match other {
                Self::Value(y) => Self::Value(x + y),
                y => y,
            },
            y => y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for NumInf<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Self::Value(x) => match other {
                Self::Value(y) => Self::Value(x - y),
                Self::Inf => Self::NegInf,
                Self::NegInf => Self::Inf,
            },
            y => y,
        }
    }
}

impl<T: Neg<Output = T>> Neg for NumInf<T> {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Self::Value(x) => Self::Value(-x),
            Self::Inf => Self::NegInf,
            Self::NegInf => Self::Inf,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // 比較
        assert!(NumInf::Value(1) < NumInf::Inf);
        assert!(NumInf::NegInf < NumInf::Value(-100));
        assert!(NumInf::<i64>::NegInf < NumInf::Inf);

        assert_eq!(NumInf::Value(1).min(NumInf::Inf), NumInf::Value(1));
        assert_eq!(NumInf::Value(1).max(NumInf::Inf), NumInf::Inf);

        // 加算
        let a = NumInf::Value(100);
        let inf = NumInf::<i64>::Inf;
        let ninf = NumInf::<i64>::NegInf;

        assert_eq!(NumInf::Value(1) + NumInf::Value(-4), NumInf::Value(-3));
        assert_eq!(inf + a, inf);
        assert_eq!(ninf + a, ninf);

        assert_eq!(a + inf, inf);
        assert_eq!(a + ninf, ninf);

        assert_eq!(inf + ninf, inf);
        assert_eq!(inf + inf, inf);
        assert_eq!(ninf + inf, ninf);
        assert_eq!(ninf + ninf, ninf);

        // 減算
        let a = NumInf::Value(100);
        let inf = NumInf::<i64>::Inf;
        let ninf = NumInf::<i64>::NegInf;

        assert_eq!(NumInf::Value(1) - NumInf::Value(-4), NumInf::Value(5));
        assert_eq!(inf - a, inf);
        assert_eq!(ninf - a, ninf);

        assert_eq!(a - inf, ninf);
        assert_eq!(a - ninf, inf);

        assert_eq!(inf - ninf, inf);
        assert_eq!(inf - inf, inf);
        assert_eq!(ninf - inf, ninf);
        assert_eq!(ninf - ninf, ninf);

        // 単項マイナス
        let inf = NumInf::<i64>::Inf;
        let ninf = NumInf::<i64>::NegInf;

        assert_eq!(-NumInf::Value(1), NumInf::Value(-1));
        assert_eq!(-inf, ninf);
        assert_eq!(-ninf, inf);
    }
}
