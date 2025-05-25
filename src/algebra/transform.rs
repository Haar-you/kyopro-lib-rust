//! 配列の並び替えを演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

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

    pub fn compose(self, other: Self) -> Self {
        let n = self.0.len();
        assert_eq!(self.0.len(), other.0.len());
        Self((0..n).map(|i| self.0[other.0[i]]).collect())
    }

    pub fn is_identity(&self) -> bool {
        for (i, &x) in self.0.iter().enumerate() {
            if i != x {
                return false;
            }
        }
        true
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

    pub fn compose(self, other: Self) -> Self {
        let n = self.0.len();
        assert_eq!(self.0.len(), other.0.len());
        Self((0..n).map(|i| self.0[other.0[i]]).collect())
    }

    pub fn inv(self) -> Self {
        let n = self.0.len();
        let mut ret = vec![0; n];
        for i in 0..n {
            ret[self.0[i]] = i;
        }
        Self(ret)
    }

    pub fn is_identity(&self) -> bool {
        for (i, &x) in self.0.iter().enumerate() {
            if i != x {
                return false;
            }
        }
        true
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
#[derive(Clone, Copy, Default, Debug)]
pub enum Composition<T> {
    #[default]
    Id,
    Value(T),
}

impl PartialEq for Composition<Transformation> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a == b,
            (Self::Value(a), _) => a.is_identity(),
            (_, Self::Value(b)) => b.is_identity(),
            _ => true,
        }
    }
}

impl PartialEq for Composition<Permutation> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a == b,
            (Self::Value(a), _) => a.is_identity(),
            (_, Self::Value(b)) => b.is_identity(),
            _ => true,
        }
    }
}

impl_algebra!(
    Composition<Transformation>;
    op: |a: Self, b: Self| {
        match (a, b) {
            (Self::Value(a), Self::Value(b)) => Self::Value(a.compose(b)),
            (a@Self::Value(_),_) => a,
            (_, b@Self::Value(_)) => b,
            _ => Self::Id
        }
    };
    id: Self::Id;
    assoc;
);

impl_algebra!(
    Composition<Permutation>;
    op: |a: Self, b: Self| {
        match (a, b) {
            (Self::Value(a), Self::Value(b)) => Self::Value(a.compose(b)),
            (a@Self::Value(_),_) => a,
            (_, b@Self::Value(_)) => b,
            _ => Self::Id
        }
    };
    inv: |a: Self| {
        match a {
            Self::Value(a) => Self::Value(a.inv()),
            _ => Self::Id
        }
    };
    id: Self::Id;
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
        let a = Composition::Value(Permutation::try_from(a).unwrap());

        let b = a.clone().inv();

        assert_eq!(a.op(b), Composition::id());
    }
}
