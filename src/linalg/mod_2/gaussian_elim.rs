//! ガウスの消去法 (mod 2)
use crate::ds::bitset::*;

/// mod 2上で行列を掃き出し、ランクを求める。
pub fn gaussian_elim(mut a: Vec<Bitset>) -> (usize, Vec<Bitset>) {
    let n = a.len();
    let Some(m) = a.first().map(|a| a.len()) else {
        return (0, a);
    };

    assert!(a.iter().all(|r| r.len() == m));
    let mut rank = 0;

    for j in 0..m {
        let mut pivot = None;

        for (i, ai) in a.iter().enumerate().skip(rank) {
            if ai.test(j) {
                pivot = Some(i);
                break;
            }
        }

        if let Some(pivot) = pivot {
            a.swap(pivot, rank);

            let ar = a.swap_remove(rank);

            for ai in a.iter_mut() {
                if ai.test(j) {
                    ai.same_size_xor_assign(&ar);
                }
            }

            a.push(ar);
            a.swap(rank, n - 1);

            rank += 1;
        } else {
            continue;
        }
    }

    (rank, a)
}
