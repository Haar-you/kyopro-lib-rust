//! 畳み込み
pub mod div_mul_transform;
pub mod mobius;
pub mod zeta;

pub mod conv_and;
pub mod conv_gcd;
pub mod conv_lcm;
pub mod conv_mul_modp;
pub mod conv_or;
pub mod conv_xor;
pub mod subset_conv;

#[cfg(test)]
mod tests {
    use crate::math::gcd_lcm::GcdLcm;
    use crate::{iter::collect::CollectVec, num::const_modint::*};
    use rand::Rng;

    use super::conv_and::convolution_and;
    use super::conv_gcd::convolution_gcd;
    use super::conv_or::convolution_or;
    use super::conv_xor::convolution_xor;
    use super::mobius::*;
    use super::subset_conv::subset_convolution;
    use super::zeta::*;

    const M: u32 = 998244353;

    fn is_subset_of(a: usize, b: usize) -> bool {
        a | b == b
    }

    #[test]
    fn test_zeta_mobius() {
        #![allow(clippy::needless_range_loop)]
        let mut rng = rand::thread_rng();

        let ff = ConstModIntBuilder::<M>;

        let n = 1 << 10;
        let f = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();

        let mut ans = vec![ff.from_u64(0); n];
        for i in 0..n {
            for j in 0..n {
                if is_subset_of(j, i) {
                    ans[i] += f[j];
                }
            }
        }

        let mut res = f.clone();
        fast_zeta_subset(&mut res);
        assert_eq!(ans, res);

        fast_mobius_subset(&mut res);
        assert_eq!(f, res);

        let mut ans = vec![ff.from_u64(0); n];
        for i in 0..n {
            for j in 0..n {
                if is_subset_of(i, j) {
                    ans[i] += f[j];
                }
            }
        }

        let mut res = f.clone();
        fast_zeta_superset(&mut res);
        assert_eq!(ans, res);

        fast_mobius_superset(&mut res);
        assert_eq!(f, res);
    }

    #[test]
    fn test_conv_or() {
        let mut rng = rand::thread_rng();

        let ff = ConstModIntBuilder::<M>;

        let n = 1 << 10;
        let f = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();
        let g = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();

        let mut ans = vec![ff.from_u64(0); n];
        for i in 0..n {
            for j in 0..n {
                ans[i | j] += f[i] * g[j];
            }
        }

        let res = convolution_or(f, g);

        assert_eq!(ans, res);
    }

    #[test]
    fn test_conv_and() {
        let mut rng = rand::thread_rng();

        let ff = ConstModIntBuilder::<M>;

        let n = 1 << 10;
        let f = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();
        let g = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();

        let mut ans = vec![ff.from_u64(0); n];
        for i in 0..n {
            for j in 0..n {
                ans[i & j] += f[i] * g[j];
            }
        }

        let res = convolution_and(f, g);

        assert_eq!(ans, res);
    }

    #[test]
    fn test_conv_xor() {
        let mut rng = rand::thread_rng();

        let ff = ConstModIntBuilder::<M>;

        let n = 1 << 10;
        let f = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();
        let g = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();

        let mut ans = vec![ff.from_u64(0); n];
        for i in 0..n {
            for j in 0..n {
                ans[i ^ j] += f[i] * g[j];
            }
        }

        let res = convolution_xor(f, g, ff);

        assert_eq!(ans, res);
    }

    #[test]
    fn test_conv_subset() {
        let mut rng = rand::thread_rng();

        let ff = ConstModIntBuilder::<M>;

        let n = 1 << 10;
        let f = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();
        let g = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n)
            .collect_vec();

        let mut ans = vec![ff.from_u64(0); n];
        for i in 0..n {
            for j in 0..n {
                if i & j == 0 {
                    ans[i | j] += f[i] * g[j];
                }
            }
        }

        let res = subset_convolution(f, g);

        assert_eq!(ans, res);
    }

    #[test]
    fn test_conv_gcd() {
        let mut rng = rand::thread_rng();

        let ff = ConstModIntBuilder::<M>;

        let n = 1000;
        let f = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n + 1)
            .collect_vec();
        let g = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..M) as u64))
            .take(n + 1)
            .collect_vec();

        let mut ans = vec![ff.from_u64(0); n + 1];
        for i in 1..=n {
            for j in 1..=n {
                ans[i.gcd(j)] += f[i] * g[j];
            }
        }

        let res = convolution_gcd(f, g);

        assert_eq!(ans[1..], res[1..]);
    }
}
