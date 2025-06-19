//! Kth root
//!
//! # Problems
//! - <https://judge.yosupo.jp/problem/kth_root_integer>

/// $\lfloor a^{1/k} \rfloor$を求める。
pub fn kth_root(a: u64, k: u64) -> u64 {
    assert!(k >= 1);

    match (a, k) {
        (0, _) => 0,
        (1, _) => 1,
        (a, 1) => a,
        (_, k) if k > 64 => 1,
        (a, k) => {
            let mut lb = 0;
            let mut ub = a;
            while ub - lb > 1 {
                let mid = midpoint(ub, lb);
                if check(mid, k, a) {
                    lb = mid;
                } else {
                    ub = mid;
                }
            }

            lb
        }
    }
}

fn check(mut a: u64, mut k: u64, n: u64) -> bool {
    let mut r: u64 = 1;

    while k > 0 {
        if k % 2 == 1 {
            let Some(_r) = r.checked_mul(a) else {
                return false;
            };
            r = _r;
        }
        if let Some(_a) = a.checked_mul(a) {
            a = _a;
        } else if k > 1 {
            return false;
        }
        k >>= 1;
    }

    r <= n
}

fn midpoint(a: u64, b: u64) -> u64 {
    ((a ^ b) >> 1) + (a & b)
}
