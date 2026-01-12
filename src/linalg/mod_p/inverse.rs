//! $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上の逆行列
use crate::num::ff::*;

/// $\mathbb{Z} / p \mathbb{Z}$($p$は素数)上での逆行列を求める。
///
/// **Time complexity** $O(n^3)$
pub fn inverse<F>(mut b: Vec<Vec<F::Element>>, modulo: &F) -> Option<Vec<Vec<F::Element>>>
where
    F: FF,
    F::Element: FFElem,
{
    let n = b.len();

    assert!(b.iter().all(|r| r.len() == n));

    for (i, bi) in b.iter_mut().enumerate() {
        bi.resize(2 * n, modulo.zero());
        bi[i + n] = modulo.one();
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
    use crate::num::modint::*;

    fn check(a: Vec<Vec<i64>>, m: u32, ans: Option<Vec<Vec<i64>>>) {
        let m = ModIntBuilder::new(m);
        let a = a
            .into_iter()
            .map(|b| b.into_iter().map(|x| m.from_i64(x)).collect())
            .collect();
        let ans = ans.map(|a| {
            a.into_iter()
                .map(|b| b.into_iter().map(|x| m.from_i64(x)).collect())
                .collect()
        });
        assert_eq!(inverse(a, &m), ans);
    }

    #[test]
    fn test() {
        check(
            vec![vec![3, 1, 4], vec![1, 5, 9], vec![2, 6, 5]],
            998244353,
            Some(vec![
                vec![188557267, 255106890, 587855008],
                vec![122007643, 987152749, 321656514],
                vec![576763404, 310564910, 976061145],
            ]),
        );
        check(
            vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            998244353,
            None,
        );
        check(
            vec![vec![0, 1], vec![1, 0]],
            998244353,
            Some(vec![vec![0, 1], vec![1, 0]]),
        );
    }
}
