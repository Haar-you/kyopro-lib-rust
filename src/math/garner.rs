//! Garner's algorithm
use crate::math::mod_ops::inv::mod_inv;

/// $$ \begin{aligned}
/// x \equiv r_1 \pmod {m_1} \\\\
/// x \equiv r_2 \pmod {m_2} \\\\
/// \vdots \\\\
/// x \equiv r_n \pmod {m_n}
/// \end{aligned} $$
/// を満たす$x \pmod {modulo}$を求める。
/// そのような$x$が存在しなければ`None`を返す。
pub fn garner(r: Vec<u64>, mut m: Vec<u64>, modulo: u64) -> Option<u64> {
    assert_eq!(r.len(), m.len());
    assert!(!r.is_empty());

    m.push(modulo);

    let n = r.len();
    let mut coeffs = vec![1; n + 1];
    let mut constants = vec![0; n + 1];

    for k in 0..n {
        let t = ((r[k] + m[k] - constants[k]) % m[k] * mod_inv(coeffs[k], m[k])?) % m[k];

        for i in k + 1..n + 1 {
            constants[i] += t * coeffs[i] % m[i];
            constants[i] %= m[i];
            coeffs[i] *= m[k];
            coeffs[i] %= m[i];
        }
    }

    Some(*constants.last().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::iter::collect::CollectVec;

    use super::*;

    #[test]
    fn test() {
        let m = vec![17, 23, 29, 35];
        let modulo = 31;

        for x in 0..=1000 {
            let r = m.iter().map(|&m| x % m).collect_vec();
            assert_eq!(garner(r, m.clone(), modulo).unwrap(), x % modulo);
        }
    }

    #[test]
    fn test_failure() {
        assert_eq!(garner(vec![1, 2], vec![6, 8], 100), None);
    }
}
