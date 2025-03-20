pub mod mobius;
pub mod zeta;

pub mod conv_and;
pub mod conv_or;
pub mod subset_conv;

#[cfg(test)]
mod tests {
    use crate::{iter::collect::CollectVec, num::const_modint::*};
    use rand::Rng;

    use super::conv_and::convolution_and;
    use super::conv_or::convolution_or;
    use super::mobius::*;
    use super::subset_conv::subset_convolution;
    use super::zeta::*;

    const M: u32 = 998244353;

    fn is_subset_of(a: usize, b: usize) -> bool {
        a | b == b
    }

    #[test]
    fn test_zeta_mobius() {
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
}
