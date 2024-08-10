use crate::utils::bitset::*;

pub fn inverse_mod_2(mut b: Vec<Bitset>) -> Option<Vec<Bitset>> {
    let n = b.len();

    assert!(b.iter().all(|r| r.len() == n));

    let mut c = vec![Bitset::new(n); n];
    for i in 0..n {
        c[i].set(i, true);
    }

    for i in 0..n {
        let q = (i..n).find(|&j| b[j].test(i))?;

        b.swap(i, q);
        c.swap(i, q);

        let bi = b.swap_remove(i);
        let ci = c.swap_remove(i);

        for (bj, cj) in b.iter_mut().zip(c.iter_mut()) {
            if bj.test(i) {
                *bj ^= bi.clone();
                *cj ^= ci.clone();
            }
        }

        b.push(bi);
        b.swap(i, n - 1);
        c.push(ci);
        c.swap(i, n - 1);
    }

    Some(c)
}