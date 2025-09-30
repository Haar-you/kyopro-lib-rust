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
pub mod sqrt_sparse;

#[cfg(test)]
mod tests {
    use super::{exp::*, inv::*, log::*, pow::*};

    use crate::math::polynomial::*;
    use crate::math::prime_mod::Prime;

    type P = Prime<998244353>;

    #[test]
    fn test_inv() {
        let a = Polynomial::<P>::from(vec![5, 4, 3, 2, 1]);
        let b = a.clone().fps_inv().unwrap();

        assert_eq!((a * b).get_until(5), Polynomial::constant(1_u32.into()));
    }

    #[test]
    fn test_log() {
        let a = Polynomial::<P>::from(vec![1, 1, 499122179, 166374064, 291154613]);
        let b = a.fps_log().unwrap();

        assert_eq!(b, vec![0, 1, 2, 3, 4].into());
    }

    #[test]
    fn test_exp() {
        let a = Polynomial::<P>::from(vec![0, 1, 2, 3, 4]);
        let b = a.clone().fps_exp().unwrap();
        let b = b.fps_log().unwrap();

        assert_eq!(b, a);
    }

    #[test]
    fn test_pow() {
        let a = Polynomial::<P>::from(vec![0, 0, 9, 2]);
        let b = a.clone().fps_pow(3).unwrap();
        assert_eq!(b, vec![0, 0, 0, 0].into());

        let a = Polynomial::<P>::from(vec![1, 1]);
        let b = a.clone().fps_pow(2).unwrap();
        assert_eq!(b, vec![1, 2].into());

        let a = Polynomial::<P>::from(vec![0, 0]);
        let b = a.clone().fps_pow(0).unwrap();
        dbg!(b);
    }
}
