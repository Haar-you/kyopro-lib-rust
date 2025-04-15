//! 配列の並び替えを演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;
use std::marker::PhantomData;

/// 変換操作
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transformation(Vec<usize>);

/// 置換操作
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Permutation(Vec<usize>);

impl Transformation {
    /// $b_i = a_{T_i}$を満たすbを返す。
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        self.0.iter().map(|&i| a[i].clone()).collect()
    }

    /// 内部の`Vec`のスライスへの参照を返す。
    pub fn as_slice(&self) -> &[usize] {
        &self.0
    }
}

impl From<Transformation> for Vec<usize> {
    fn from(value: Transformation) -> Self {
        value.0
    }
}

impl TryFrom<Vec<usize>> for Transformation {
    type Error = &'static str;

    fn try_from(value: Vec<usize>) -> Result<Self, Self::Error> {
        let n = value.len();
        value
            .iter()
            .all(|&i| i < n)
            .then_some(Self(value))
            .ok_or("すべての値は`.len()`未満でなければならない。")
    }
}

impl Permutation {
    /// $b_i = a_{P_i}$を満たすbを返す。
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        self.0.iter().map(|&i| a[i].clone()).collect()
    }

    /// 内部の`Vec`のスライスへの参照を返す。
    pub fn as_slice(&self) -> &[usize] {
        &self.0
    }
}

impl From<Permutation> for Vec<usize> {
    fn from(value: Permutation) -> Self {
        value.0
    }
}

impl TryFrom<Vec<usize>> for Permutation {
    type Error = &'static str;

    fn try_from(value: Vec<usize>) -> Result<Self, Self::Error> {
        let mut check = vec![false; value.len()];

        for &x in &value {
            if x >= value.len() || check[x] {
                return Err("0から`.len()｀未満の値からなる順列でなければならない。");
            }
            check[x] = true;
        }
        Ok(Self(value))
    }
}

/// 置換や変換の合成を演算とする代数的構造
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Composition<T> {
    len: usize,
    phantom: PhantomData<T>,
}

impl<T> Composition<T> {
    /// `Composition<T>`を生成する。
    pub fn new(len: usize) -> Self {
        Self {
            len,
            phantom: PhantomData,
        }
    }
}

impl_algebra!(
    Composition<Transformation>;
    set: Transformation;
    op: |s: &Self, a: Transformation, b: Transformation| {
        let n = s.len;
        assert_eq!(a.0.len(), n);
        assert_eq!(b.0.len(), n);
        Transformation((0..n).map(|i| a.0[b.0[i]]).collect())
    };
    id: |s: &Self| Transformation((0..s.len).collect());
    assoc;
);

impl_algebra!(
    Composition<Permutation>;
    set: Permutation;
    op: |s: &Self, a: Permutation, b: Permutation| {
        let n = s.len;
        assert_eq!(a.0.len(), n);
        assert_eq!(b.0.len(), n);
        Permutation((0..n).map(|i| a.0[b.0[i]]).collect())
    };
    inv: |s: &Self, a: Permutation| {
        let n = s.len;
        assert_eq!(a.0.len(), n);
        let mut ret = vec![0; n];
        for i in 0..n { ret[a.0[i]] = i; }
        Permutation(ret)
    };
    id: |s: &Self| Permutation((0..s.len).collect());
    assoc;
);

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 100;
        let m = Composition::new(n);

        let mut a = (0..n).collect::<Vec<_>>();
        a.shuffle(&mut rng);
        let a = Permutation::try_from(a).unwrap();

        let b = m.inv(a.clone());

        assert_eq!(m.op(a, b), m.id());
    }
}
