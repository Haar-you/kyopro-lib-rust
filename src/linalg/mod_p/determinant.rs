use crate::math::mod_ops::inv_p::mod_inv_p;

/// 素数mod p上での行列式を求める。
/// $O(n^3)$
#[inline]
pub fn determinant(mut a: Vec<Vec<u64>>, p: u64) -> u64 {
    let n = a.len();

    assert!(a.iter().all(|r| r.len() == n));

    let mut s = 0;
    for i in 0..n {
        if a[i][i] == 0 {
            if let Some(j) = (i + 1..n).find(|&j| a[j][i] != 0) {
                a.swap(i, j);
                s ^= 1;
            } else {
                return 0;
            }
        }

        let d = mod_inv_p(a[i][i], p);
        let ai = a.swap_remove(i);

        for aj in a.iter_mut().skip(i) {
            let t = aj[i] * d % p;
            for (x, y) in aj.iter_mut().zip(ai.iter()) {
                *x = (*x + p - *y * t % p) % p;
            }
        }

        a.push(ai);
        a.swap(i, n - 1);
    }

    let mut ret = 1;
    for (i, a) in a.into_iter().enumerate() {
        ret = (ret * a[i]) % p;
    }

    if s == 1 && ret != 0 {
        ret = p - ret;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = 998244353;

        let a = vec![vec![3, 1, 4], vec![1, 5, 9], vec![2, 6, 5]];
        assert_eq!(determinant(a, p), 998244263);

        let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(determinant(a, p), 0);

        let a = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(determinant(a, p), 998244352);
    }
}
