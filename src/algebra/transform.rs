//! 配列の並び替えを演算とする代数的構造
pub use crate::algebra::traits::*;
use crate::impl_algebra;

/// 変換操作
#[derive(Clone, Debug, Default)]
pub enum Transformation {
    #[default]
    Id,
    Value(Vec<usize>),
}

impl Transformation {
    /// $b_i = a_{T_i}$を満たすbを返す。
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        match self {
            Self::Id => a,
            Self::Value(t) => t.iter().map(|&i| a[i].clone()).collect(),
        }
    }

    pub fn is_identity(&self) -> bool {
        match self {
            Self::Id => true,
            Self::Value(a) => a.iter().enumerate().all(|(i, &x)| i == x),
        }
    }

    /// 内部の`Vec`のスライスへの参照を返す。
    pub fn as_slice(&self) -> Option<&[usize]> {
        match self {
            Self::Id => None,
            Self::Value(a) => Some(a),
        }
    }

    pub fn compose(self, other: Self) -> Self {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => {
                let n = a.len();
                assert_eq!(a.len(), b.len());
                Self::Value((0..n).map(|i| a[b[i]]).collect())
            }
            (a @ Self::Value(_), _) => a,
            (_, b @ Self::Value(_)) => b,
            _ => Self::Id,
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
            .then_some(Self::Value(value))
            .ok_or("すべての値は`.len()`未満でなければならない。")
    }
}

impl PartialEq for Transformation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a == b,
            _ => self.is_identity() && other.is_identity(),
        }
    }
}

impl_algebra!(
    Transformation;
    op: |a: Self, b: Self| a.compose(b);
    id: Self::Id;
    assoc;
);

/// 置換操作
#[derive(Clone, Debug, Default)]
pub enum Permutation {
    #[default]
    Id,
    Value(Vec<usize>),
}

impl Permutation {
    /// $b_i = a_{P_i}$を満たすbを返す。
    pub fn apply<T: Clone>(&self, a: Vec<T>) -> Vec<T> {
        match self {
            Self::Id => a,
            Self::Value(t) => t.iter().map(|&i| a[i].clone()).collect(),
        }
    }

    pub fn is_identity(&self) -> bool {
        match self {
            Self::Id => true,
            Self::Value(a) => a.iter().enumerate().all(|(i, &x)| i == x),
        }
    }

    /// 内部の`Vec`のスライスへの参照を返す。
    pub fn as_slice(&self) -> Option<&[usize]> {
        match self {
            Self::Id => None,
            Self::Value(a) => Some(a),
        }
    }

    pub fn compose(self, other: Self) -> Self {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => {
                let n = a.len();
                assert_eq!(a.len(), b.len());
                Self::Value((0..n).map(|i| a[b[i]]).collect())
            }
            (a @ Self::Value(_), _) => a,
            (_, b @ Self::Value(_)) => b,
            _ => Self::Id,
        }
    }

    pub fn inv(self) -> Self {
        match self {
            Self::Id => self,
            Self::Value(a) => {
                let n = a.len();
                let mut ret = vec![0; n];
                for i in 0..n {
                    ret[a[i]] = i;
                }
                Self::Value(ret)
            }
        }
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
        Ok(Self::Value(value))
    }
}

impl PartialEq for Permutation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a == b,
            _ => self.is_identity() && other.is_identity(),
        }
    }
}

impl_algebra!(
    Permutation;
    op: |a: Self, b: Self| a.compose(b);
    inv: |a: Self| a.inv();
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
        let a = Permutation::try_from(a).unwrap();

        let b = a.clone().inv();

        assert_eq!(a.op(b), Permutation::id());
    }
}
