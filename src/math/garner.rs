//! Garner's algorithm
use crate::math::mod_ops::inv::mod_inv;

pub fn garner(r: Vec<u64>, mut m: Vec<u64>, modulo: u64) -> u64 {
    assert_eq!(r.len(), m.len());

    m.push(modulo);

    let n = r.len();
    let mut coeffs = vec![1; n + 1];
    let mut constants = vec![0; n + 1];

    for k in 0..n {
        let t = ((r[k] + m[k] - constants[k]) % m[k] * mod_inv(coeffs[k], m[k]).unwrap()) % m[k];

        for i in k + 1..n + 1 {
            constants[i] += t * coeffs[i] % m[i];
            constants[i] %= m[i];
            coeffs[i] *= m[k];
            coeffs[i] %= m[i];
        }
    }

    *constants.last().unwrap()
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
            assert_eq!(garner(r, m.clone(), modulo), x % modulo);
        }
    }
}
