//! 逆行列 (mod 素数)
use crate::math::mod_ops::inv_p::mod_inv_p;

/// 素数mod p上での逆行列を求める。
///
/// **Time complexity** $O(n^3)$
pub fn inverse(mut b: Vec<Vec<u64>>, p: u64) -> Option<Vec<Vec<u64>>> {
    let n = b.len();

    assert!(b.iter().all(|r| r.len() == n));

    for (i, bi) in b.iter_mut().enumerate() {
        bi.resize(2 * n, 0);
        bi[i + n] = 1;
    }

    for i in 0..n {
        let q = (i..n).find(|&j| b[j][i] != 0)?;

        b.swap(i, q);

        let d = mod_inv_p(b[i][i], p);

        for x in b[i].iter_mut() {
            *x = *x * d % p;
        }

        let d = mod_inv_p(b[i][i], p);

        let bi = b.swap_remove(i);

        for bj in b.iter_mut() {
            let d = bj[i] * d % p;

            for (x, y) in bj.iter_mut().zip(bi.iter()) {
                let t = *y * d % p;
                if *x >= t {
                    *x -= t;
                } else {
                    *x += p - t;
                }
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

    #[test]
    fn test() {
        let p = 998244353;

        let a = vec![vec![3, 1, 4], vec![1, 5, 9], vec![2, 6, 5]];
        assert_eq!(
            inverse(a, p),
            Some(vec![
                vec![188557267, 255106890, 587855008],
                vec![122007643, 987152749, 321656514],
                vec![576763404, 310564910, 976061145]
            ])
        );

        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(inverse(a, p), None);

        let a = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(inverse(a, p), Some(vec![vec![0, 1], vec![1, 0]]))
    }
}
