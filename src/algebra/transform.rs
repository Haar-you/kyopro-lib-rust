//! 配列の並び替えを演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 変換操作
#[derive(Clone, Debug, Default)]
pub struct Transformation {
    value: Option<Vec<usize>>,
}

impl Transformation {
    /// `i`番目の要素を返す。
    pub fn get(&self, i: usize) -> usize {
        self.value.as_ref().map_or_else(|| i, |a| a[i])
    }

    /// $b_i = a_{T_i}$を満たすbを返す。
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        self.value
            .as_ref()
            .map(|t| t.iter().map(|&i| a[i].clone()).collect())
            .unwrap_or(a)
    }

    /// 単位元であるとき、`true`を返す。
    pub fn is_identity(&self) -> bool {
        self.value
            .as_ref()
            .map_or(true, |a| a.iter().enumerate().all(|(i, &x)| i == x))
    }

    /// 操作を合成する。
    pub fn compose(self, other: Self) -> Self {
        Self {
            value: match (self.value, other.value) {
                (Some(a), Some(b)) => {
                    let n = a.len();
                    assert_eq!(a.len(), b.len());
                    Some((0..n).map(|i| a[b[i]]).collect())
                }
                (a @ Some(_), _) => a,
                (_, b @ Some(_)) => b,
                _ => None,
            },
        }
    }
}

impl TryFrom<Vec<usize>> for Transformation {
    type Error = &'static str;

    fn try_from(value: Vec<usize>) -> Result<Self, Self::Error> {
        let n = value.len();
        value
            .iter()
            .all(|&i| i < n)
            .then_some(Self { value: Some(value) })
            .ok_or("すべての値は`.len()`未満でなければならない。")
    }
}

impl PartialEq for Transformation {
    fn eq(&self, other: &Self) -> bool {
        match (&self.value, &other.value) {
            (Some(a), Some(b)) => a == b,
            _ => self.is_identity() && other.is_identity(),
        }
    }
}

impl_algebra!(
    Transformation;
    op: |a: Self, b: Self| a.compose(b);
    id: Self { value: None };
    assoc;
);
