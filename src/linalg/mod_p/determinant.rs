//! $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上の行列式
use crate::num::ff::*;

/// $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上で行列式を求める。
///
/// **Time complexity** $O(n^3)$
pub fn determinant<F>(mut a: Vec<Vec<F::Element>>, modulo: &F) -> F::Element
where
    F: FF,
    F::Element: FFElem,
{
    let n = a.len();

    assert!(a.iter().all(|r| r.len() == n));

    let mut minus = false;
    for i in 0..n {
        if a[i][i].value() == 0 {
            if let Some(j) = (i + 1..n).find(|&j| a[j][i].value() != 0) {
                a.swap(i, j);
                minus = !minus;
            } else {
                return modulo.zero();
            }
        }

        let d = a[i][i].inv();
        let ai = a.swap_remove(i);

        for aj in a.iter_mut().skip(i) {
            let t = aj[i] * d;
            for (x, y) in aj.iter_mut().skip(i).zip(ai.iter().skip(i)) {
                *x -= *y * t;
            }
        }

        a.push(ai);
        a.swap(i, n - 1);
    }

    let mut ret = modulo.one();
    for (i, a) in a.into_iter().enumerate() {
        ret *= a[i];
    }

    if minus {
        ret = -ret;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::prime_mod::Prime, num::const_modint::*};

    fn convert<U, T>(a: Vec<Vec<T>>) -> Vec<Vec<U>>
    where
        U: From<T>,
    {
        a.into_iter()
            .map(|b| b.into_iter().map(From::from).collect())
            .collect()
    }

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let m = ConstModIntBuilder::<P>::new();

        let a = vec![vec![3, 1, 4], vec![1, 5, 9], vec![2, 6, 5]];
        let a = convert::<ConstModInt<P>, _>(a);
        assert_eq!(determinant(a, &m).value(), 998244263);

        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let a = convert::<ConstModInt<P>, _>(a);
        assert_eq!(determinant(a, &m).value(), 0);

        let a = vec![vec![0, 1], vec![1, 0]];
        let a = convert::<ConstModInt<P>, _>(a);
        assert_eq!(determinant(a, &m).value(), 998244352);
    }
}
