//! 商の列挙

/// [`enumerate_quotients`]の結果
#[derive(Copy, Debug, Eq, PartialEq, Clone)]
pub struct Quotient {
    /// `floor(N/x)`の値
    pub q: u64,
    /// `x`の最小値
    pub from: u64,
    /// `x`の最大値
    pub to: u64,
}

/// 1以上N以下の自然数xについて`floor(N/x)`の取りうる値とそれを与えるxの範囲を列挙する。
///
/// **Time complexity** $O(\sqrt{N})$
pub fn enumerate_quotients(n: u64) -> Vec<Quotient> {
    let mut ret = vec![];

    let mut k = 1;

    while k * k <= n {
        let q = n / k;
        ret.push(Quotient { q, from: k, to: k });

        k += 1;
    }

    while k <= n {
        let q = n / k;
        let u = n / q;
        ret.push(Quotient { q, from: k, to: u });

        k = u + 1;
    }

    ret.reverse();

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for n in 1..200 {
            let ans = enumerate_quotients(n);
            for Quotient { q, from, to } in ans {
                assert_eq!(q, n / from);
                assert_eq!(q, n / to);
                assert_ne!(q, n / (to + 1));
            }
        }
    }
}
