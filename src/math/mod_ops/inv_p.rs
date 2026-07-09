//! mod p(素数)での逆元

/// $ax \equiv 1 \pmod p$を満たす$x$を求める。
///
/// $p$は$0 < p < 2^{32}$の範囲の素数であること。
///
/// **Time complexity** $O(\log p)$
pub fn mod_inv_p(mut a: u64, p: u64) -> u64 {
    assert!(
        0 < p && p <= 0xFFFFFFFF,
        "Violated 0 < p <= 0xFFFFFFFF (p = {p})"
    );
    if a >= p {
        a %= p;
    }

    let mut b = p;
    let mut u = 1;
    let mut v = 0;

    while b > 0 {
        let t = a / b;

        a -= t * b;
        (a, b) = (b, a);

        if u < t * v {
            u += p - (t * v) % p;
            if u >= p {
                u -= p;
            }
        } else {
            u -= t * v;
        }
        (u, v) = (v, u);
    }

    u
}
