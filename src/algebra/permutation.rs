//! 配列の置換の合成
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 置換操作
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Permutation {
    value: Vec<usize>,
}

impl Permutation {
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

    /// $b_i = a_{P_i}$を満たすbを返す。
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

    /// 逆操作を返す。
    pub fn inv(self) -> Self {
        let n = self.value.len();
        let mut ret = vec![0; n];
        for i in 0..n {
            ret[self.value[i]] = i;
        }
        Self { value: ret }
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
        Ok(Self { value })
    }
}

/// [`Permutation`]の合成
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Composition(pub usize);

impl_algebra!(
    Composition;
    set: Permutation;
    op: |_, a: Permutation, b: Permutation| a.compose(b);
    inv: |_, a: Permutation| a.inv();
    id: |s: &Self| Permutation::id(s.0);
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

        let mut a = (0..n).collect::<Vec<_>>();
        a.shuffle(&mut rng);
        let a = Permutation::try_from(a).unwrap();

        let b = a.clone().inv();

        let m = Composition(n);
        assert_eq!(m.op(a, b), m.id());
    }
}
