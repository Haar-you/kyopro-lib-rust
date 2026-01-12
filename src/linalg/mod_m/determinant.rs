//! $\mathbb{Z} / m \mathbb{Z}$上の行列式
use crate::num::zz::{ZZElem, ZZ};

/// $\mathbb{Z} / m \mathbb{Z}$上で行列式を求める。
pub fn determinant<R>(mut a: Vec<Vec<R::Element>>, ring: R) -> R::Element
where
    R: ZZ + Copy,
    R::Element: ZZElem,
{
    let n = a.len();

    assert!(a.iter().all(|r| r.len() == n));

    let mut minus = false;

    for i in 0..n {
        if a[i][i].value() == 0 {
            if let Some(j) = (i + 1..n).find(|&j| a[j][i].value() != 0) {
                a.swap(i, j);
                minus = !minus;
            } else {
                return ring.zero();
            }
        }

        let mut ai = a.swap_remove(i);

        for aj in a.iter_mut().skip(i) {
            let t = aj[i];
            if t.value() == 0 {
                continue;
            }

            loop {
                if aj[i].value() == 0 {
                    break;
                }

                if ai[i].value() > aj[i].value() {
                    std::mem::swap(&mut ai, aj);
                    minus = !minus;
                }

                let t = ring.from_u64((aj[i].value() / ai[i].value()) as u64);

                for (x, y) in aj.iter_mut().skip(i).zip(ai.iter().skip(i)) {
                    *x -= *y * t;
                }
            }
        }

        a.push(ai);
        a.swap(i, n - 1);
    }

    let mut ret = ring.one();
    for (i, ai) in a.into_iter().enumerate() {
        ret *= ai[i];
    }

    if minus {
        ret = -ret;
    }

    ret
}
