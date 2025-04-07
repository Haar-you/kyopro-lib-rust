//! ガウスの消去法 (mod p)
use crate::num::ff::FFElem;

/// mod p上で行列を掃き出し、ランクを求める。
pub fn gaussian_elim<T>(mut a: Vec<Vec<T>>) -> (usize, Vec<Vec<T>>)
where
    T: FFElem + Copy,
{
    let n = a.len();
    let Some(m) = a.first().map(|a| a.len()) else {
        return (0, a);
    };

    assert!(a.iter().all(|r| r.len() == m));
    let mut rank = 0;

    for j in 0..m {
        let mut pivot = None;

        for (i, ai) in a.iter().enumerate().skip(rank) {
            if ai[j].value() != 0 {
                pivot = Some(i);
                break;
            }
        }

        if let Some(pivot) = pivot {
            a.swap(pivot, rank);

            let mut ar = a.swap_remove(rank);
            let d = ar[j].inv();
            for x in ar.iter_mut() {
                *x *= d;
            }

            for ai in a.iter_mut() {
                let d = ai[j];
                if d.value() != 0 {
                    for (aij, arj) in ai.iter_mut().zip(ar.iter()) {
                        *aij -= *arj * d;
                    }
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
