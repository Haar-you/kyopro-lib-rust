//! 配列の並び替えの合成
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 変換操作
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Transformation {
    value: Vec<usize>,
}

impl Transformation {
    /// 恒等変換を返す。
    pub fn id(n: usize) -> Self {
        Self {
            value: (0..n).collect(),
        }
    }

    /// `i`番目の要素を返す。
    pub fn get(&self, i: usize) -> usize {
        self.value[i]
    }

    /// $b_i = a_{T_i}$を満たすbを返す。
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        self.value.iter().map(|&i| a[i].clone()).collect()
    }

    /// 操作を合成する。
    pub fn compose(self, other: Self) -> Self {
        let (a, b) = (self.value, other.value);
        let n = a.len();
        assert_eq!(a.len(), b.len());
        Self {
            value: (0..n).map(|i| a[b[i]]).collect(),
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
            .then_some(Self { value })
            .ok_or("すべての値は`.len()`未満でなければならない。")
    }
}

/// [`Transformation`]の合成
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Composition(pub usize);

impl_algebra!(
    Composition;
    set: Transformation;
    op: |_, a: Transformation, b: Transformation| a.compose(b);
    id: |s: &Self| Transformation::id(s.0);
    assoc;
);
