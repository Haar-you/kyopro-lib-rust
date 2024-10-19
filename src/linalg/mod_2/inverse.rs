use crate::ds::bitset::*;

/// mod 2上で逆行列を求める
pub fn inverse(mut b: Vec<Bitset>) -> Option<Vec<Bitset>> {
    let n = b.len();

    assert!(b.iter().all(|r| r.len() == n));

    let mut c = vec![Bitset::new(n); n];

    for (i, c) in c.iter_mut().enumerate() {
        c.set(i, true);
    }

    for i in 0..n {
        let q = (i..n).find(|&j| b[j].test(i))?;

        b.swap(i, q);
        c.swap(i, q);

        let bi = b.swap_remove(i);
        let ci = c.swap_remove(i);

        for (bj, cj) in b.iter_mut().zip(c.iter_mut()) {
            if bj.test(i) {
                bj.same_size_xor_assign(&bi);
                cj.same_size_xor_assign(&ci);
            }
        }

        b.push(bi);
        b.swap(i, n - 1);
        c.push(ci);
        c.swap(i, n - 1);
    }

    Some(c)
}
