//! グループ分けの方法の全列挙
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc390/tasks/abc390_d>

use crate::algo::enumerate_bitset::subset_asc::*;

/// `n`個の区別できるものをグループ分けする方法をすべて列挙する。
/// グループ分けの方法の個数はベル数となる。
///
/// `proc`はグループ分けの結果を受け取って処理する。
/// 一つのグループは、`i`番目の要素が含まれていれば、`i`番目のビットが立っているような、`u32`で返される。
pub fn enum_groups<F>(n: usize, mut proc: F)
where
    F: FnMut(&Vec<u32>),
{
    rec(n, 0, &mut proc, &mut vec![]);
}

fn rec<F>(n: usize, bit: u32, proc: &mut F, gs: &mut Vec<u32>)
where
    F: FnMut(&Vec<u32>),
{
    let mask = (1 << n) - 1;
    if bit == mask {
        proc(gs);
    } else {
        let left_unused = bit.trailing_ones();
        let left = 1 << left_unused;
        for rest in subset_asc((mask & !bit) ^ left) {
            let g = rest | left;
            gs.push(g);
            rec(n, bit | g, proc, gs);
            gs.pop();
        }
    }
}
