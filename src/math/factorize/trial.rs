//! 試し割り素因数分解

/// 試し割り素因数分解
///
/// **Time comlexity** $\sqrt{n}$
pub fn factorize(mut n: u64) -> Vec<(u64, usize)> {
    let mut ret = vec![];

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let mut c = 0;
            while n % i == 0 {
                n /= i;
                c += 1;
            }
            ret.push((i, c));
        }
        i += 1
    }

    if n != 1 {
        ret.push((n, 1));
    }

    ret
}
