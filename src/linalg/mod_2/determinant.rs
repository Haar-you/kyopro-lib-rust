use crate::ds::bitset::*;

/// mod 2上で行列式を求める
pub fn determinant(mut a: Vec<Bitset>) -> u64 {
    let n = a.len();

    assert!(a.iter().all(|r| r.len() == n));

    for i in 0..n {
        if !a[i].test(i) {
            if let Some(j) = (i + 1..n).find(|&j| a[j].test(i)) {
                a.swap(i, j);
            } else {
                return 0;
            }
        }
        let ai = a.swap_remove(i);

        for aj in a.iter_mut().skip(i) {
            if aj.test(i) {
                aj.same_size_xor_assign(&ai);
            }
        }

        a.push(ai);
        a.swap(i, n - 1);
    }

    1
}
