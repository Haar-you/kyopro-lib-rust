//! 分割数$p(0), \dots, p(n)$を列挙する。
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/partition_function>
use crate::math::prime_mod::PrimeMod;
use crate::{
    math::{fps::inv::*, polynomial::*},
    num::const_modint::*,
};

/// 分割数$p(0), \dots, p(n)$を列挙する。
pub fn partition_number<P: PrimeMod>(n: usize) -> Vec<ConstModInt<P>> {
    let ff = ConstModIntBuilder::<P>::new();
    let mut f = vec![ff.from_u64(0); n + 1];
    f[0] = ff.from_u64(1);

    let m = ((1 + 24 * n).isqrt() - 1) / 6;
    for i in 1..=m {
        f[i * (3 * i + 1) / 2] += ff.from_i64(if i % 2 == 0 { 1 } else { -1 });
    }

    let m = ((1 + 24 * n).isqrt() + 1) / 6;
    for i in 1..=m {
        f[i * (3 * i - 1) / 2] += ff.from_i64(if i % 2 == 0 { 1 } else { -1 });
    }

    Polynomial::from(f).fps_inv().unwrap().into()
}

#[cfg(test)]
mod tests {
    use crate::math::prime_mod::Prime;

    use super::*;

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let res = partition_number::<P>(49);

        let ans = [
            1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627,
            792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604, 6842, 8349, 10143, 12310,
            14883, 17977, 21637, 26015, 31185, 37338, 44583, 53174, 63261, 75175, 89134, 105558,
            124754, 147273, 173525,
        ];

        assert_eq!(res, ans.map(|x| x.into()));
    }
}
