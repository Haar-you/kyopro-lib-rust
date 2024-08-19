use crate::ds::bitset::*;

pub fn determinant_mod_2(mut a: Vec<Bitset>) -> u64 {
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
                *aj ^= ai.clone();
            }
        }

        a.push(ai);
        a.swap(i, n - 1);
    }

    1
}
