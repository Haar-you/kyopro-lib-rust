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
    pub fn try_from(a: Vec<usize>) -> Option<Self> {
        let n = a.len();
        a.iter().all(|&i| i < n).then_some(Self(a))
    }
    pub fn into_inner(self) -> Vec<usize> {
        self.0
    }
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        self.0.iter().map(|&i| a[i].clone()).collect()
    }
}

impl Permutation {
    pub fn try_from(a: Vec<usize>) -> Option<Self> {
        let mut check = vec![false; a.len()];

        for &x in &a {
            if x >= a.len() || check[x] {
                return None;
            }
            check[x] = true;
        }
        Some(Self(a))
    }
    pub fn into_inner(self) -> Vec<usize> {
        self.0
    }
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        self.0.iter().map(|&i| a[i].clone()).collect()
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

impl_algebra!(Composition<Transformation>, 
    set: Transformation,
    op: |s: &Self, a: Transformation, b: Transformation| {
        let n = s.len;
        assert_eq!(a.0.len(), n);
        assert_eq!(b.0.len(), n);
        Transformation((0..n).map(|i| a.0[b.0[i]]).collect())
    },
    id: |s: &Self| Transformation((0..s.len).collect()), assoc: {});

impl_algebra!(Composition<Permutation>, 
    set: Permutation,
    op: |s: &Self, a: Permutation, b: Permutation| {
        let n = s.len;
        assert_eq!(a.0.len(), n);
        assert_eq!(b.0.len(), n);
        Permutation((0..n).map(|i| a.0[b.0[i]]).collect())
    }, 
    inv: |s: &Self, a: Permutation| {
        let n = s.len;
        assert_eq!(a.0.len(), n);
        let mut ret = vec![0; n];
        for i in 0..n { ret[a.0[i]] = i; }
        Permutation(ret)
    },
    id: |s: &Self| Permutation((0..s.len).collect()), assoc: {});

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
