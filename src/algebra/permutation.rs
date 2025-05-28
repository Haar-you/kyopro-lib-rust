//! 配列の置換を演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 置換操作
#[derive(Clone, Debug, Default)]
pub struct Permutation {
    value: Option<Vec<usize>>,
}

impl Permutation {
    /// `i`番目の要素を返す。
    pub fn get(&self, i: usize) -> usize {
        self.value.as_ref().map_or_else(|| i, |a| a[i])
    }

    /// $b_i = a_{P_i}$を満たすbを返す。
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
            .is_none_or(|a| a.iter().enumerate().all(|(i, &x)| i == x))
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
                (a, None) => a,
                (None, b) => b,
            },
        }
    }

    /// 逆操作を返す。
    pub fn inv(self) -> Self {
        self.value
            .as_ref()
            .map(|a| {
                let n = a.len();
                let mut ret = vec![0; n];
                for i in 0..n {
                    ret[a[i]] = i;
                }
                Self { value: Some(ret) }
            })
            .unwrap_or(self)
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
        Ok(Self { value: Some(value) })
    }
}

impl PartialEq for Permutation {
    fn eq(&self, other: &Self) -> bool {
        match (&self.value, &other.value) {
            (Some(a), Some(b)) => a == b,
            _ => self.is_identity() && other.is_identity(),
        }
    }
}

impl_algebra!(
    Permutation;
    op: |a: Self, b: Self| a.compose(b);
    inv: |a: Self| a.inv();
    id: Self { value: None };
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

        assert_eq!(a.op(b), Permutation::id());
    }
}
