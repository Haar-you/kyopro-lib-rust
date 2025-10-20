//! 逆行列 (mod 素数)
use crate::num::{ff::FFElem, one_zero::*};

/// 素数mod p上での逆行列を求める。
///
/// **Time complexity** $O(n^3)$
pub fn inverse<T>(mut b: Vec<Vec<T>>) -> Option<Vec<Vec<T>>>
where
    T: FFElem + Copy + Zero + One,
{
    let n = b.len();

    assert!(b.iter().all(|r| r.len() == n));

    for (i, bi) in b.iter_mut().enumerate() {
        bi.resize(2 * n, T::zero());
        bi[i + n] = T::one();
    }

    for i in 0..n {
        let q = (i..n).find(|&j| b[j][i].value() != 0)?;

        b.swap(i, q);

        let d = b[i][i].inv();

        for x in b[i].iter_mut() {
            *x *= d;
        }

        let d = b[i][i].inv();

        let bi = b.swap_remove(i);

        for bj in b.iter_mut() {
            let d = bj[i] * d;

            for (x, y) in bj.iter_mut().zip(bi.iter()) {
                *x -= *y * d;
            }
        }

        b.push(bi);
        b.swap(i, n - 1);
    }

    let ret = b.into_iter().map(|a| a[n..].to_vec()).collect();

    Some(ret)
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
        let a = vec![vec![3, 1, 4], vec![1, 5, 9], vec![2, 6, 5]];
        let a = convert::<ConstModInt<P>, _>(a);
        let res = inverse(a);
        let res = res.map(convert::<u32, _>);
        assert_eq!(
            res,
            Some(vec![
                vec![188557267, 255106890, 587855008],
                vec![122007643, 987152749, 321656514],
                vec![576763404, 310564910, 976061145]
            ])
        );

        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let a = convert::<ConstModInt<P>, _>(a);
        let res = inverse(a);
        let res = res.map(convert::<u32, _>);
        assert_eq!(res, None);

        let a = vec![vec![0, 1], vec![1, 0]];
        let a = convert::<ConstModInt<P>, _>(a);
        let res = inverse(a);
        let res = res.map(convert::<u32, _>);
        assert_eq!(res, Some(vec![vec![0, 1], vec![1, 0]]));
    }
}
