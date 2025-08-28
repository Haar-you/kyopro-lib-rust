//! Formal Power Series
//!
//! # References
//! - <https://maspypy.com/%E5%A4%9A%E9%A0%85%E5%BC%8F%E3%83%BB%E5%BD%A2%E5%BC%8F%E7%9A%84%E3%81%B9%E3%81%8D%E7%B4%9A%E6%95%B0%E6%95%B0%E3%81%88%E4%B8%8A%E3%81%92%E3%81%A8%E3%81%AE%E5%AF%BE%E5%BF%9C%E4%BB%98%E3%81%91>
//! - <https://qiita.com/hotman78/items/f0e6d2265badd84d429a>
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/inv_of_formal_power_series>
//! - <https://judge.yosupo.jp/problem/log_of_formal_power_series>
//! - <https://judge.yosupo.jp/problem/exp_of_formal_power_series>
//! - <https://judge.yosupo.jp/problem/pow_of_formal_power_series>

pub mod exp;
pub mod inv;
pub mod log;
pub mod pow;
pub mod sqrt;

pub mod exp_sparse;
pub mod inv_sparse;
pub mod log_sparse;
pub mod pow_sparse;

#[cfg(test)]
mod tests {
    use super::{exp::*, inv::*, log::*, pow::*};

    use crate::math::ntt::*;
    use crate::math::polynomial::*;

    #[test]
    fn test_inv() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![5, 4, 3, 2, 1];
        let b = po.fps_inv(a.clone().into());

        assert_eq!(
            po.mul(a.into(), b).get_until(5),
            Polynomial::constant(1_u32.into())
        );
    }

    #[test]
    fn test_log() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![1, 1, 499122179, 166374064, 291154613];
        let b = po.fps_log(a.clone().into());

        assert_eq!(b, vec![0, 1, 2, 3, 4].into());
    }

    #[test]
    fn test_exp() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![0, 1, 2, 3, 4];
        let b = po.fps_exp(a.clone().into());
        let b = po.fps_log(b);

        assert_eq!(b, a.into());
    }

    #[test]
    fn test_pow() {
        let ntt = NTT998244353::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<u32> = vec![0, 0, 9, 2];
        let b = po.fps_pow(a.clone().into(), 3);
        assert_eq!(b, vec![0, 0, 0, 0].into());

        let a: Vec<u32> = vec![1, 1];
        let b = po.fps_pow(a.clone().into(), 2);
        assert_eq!(b, vec![1, 2].into());

        let a: Vec<u32> = vec![0, 0];
        let b = po.fps_pow(a.clone().into(), 0);
        dbg!(b);
    }
}
