//! 循環検出法
//!
//! # References
//! - <https://ja.wikipedia.org/wiki/%E3%83%95%E3%83%AD%E3%82%A4%E3%83%89%E3%81%AE%E5%BE%AA%E7%92%B0%E6%A4%9C%E5%87%BA%E6%B3%95>
//!
//! # Problems
//! - <https://atcoder.jp/contests/abc179/tasks/abc179_e>
//! - <https://atcoder.jp/contests/typical90/tasks/typical90_bf>

/// [`cycle_finding`]の結果
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rho {
    /// 先頭の非循環部の長さ
    pub tail: u64,
    /// 循環部の長さ
    pub cycle: u64,
}

/// 循環検出法
///
/// **Space complexity** $O(1)$
pub fn cycle_finding<T>(init: T, f: impl Fn(T) -> T) -> Rho
where
    T: Copy + Eq,
{
    let mut a = init;
    let mut b = init;
    loop {
        a = f(a);
        b = f(f(b));

        if a == b {
            break;
        }
    }

    let mut tail = 0;
    let mut c = init;
    while a != c {
        c = f(c);
        a = f(a);
        tail += 1;
    }

    let mut cycle = 0;
    loop {
        a = f(a);
        c = f(f(c));
        cycle += 1;

        if a == c {
            break;
        }
    }

    Rho { tail, cycle }
}
