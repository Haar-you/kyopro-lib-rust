//! 原始根

use crate::math::{factorize::trial::*, mod_ops::pow::*};

pub fn primitive_root(p: u64) -> Option<u64> {
    let pf = factorize(p - 1)
        .into_iter()
        .map(|(x, _)| x)
        .collect::<Vec<_>>();

    for g in 2..=p {
        if pf.iter().all(|f| mod_pow(g, (p - 1) / f, p) != 1) {
            return Some(g);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(primitive_root(469762049), Some(3));
        assert_eq!(primitive_root(167772161), Some(3));
        assert_eq!(primitive_root(754974721), Some(11));
        assert_eq!(primitive_root(1012924417), Some(5));
    }
}
