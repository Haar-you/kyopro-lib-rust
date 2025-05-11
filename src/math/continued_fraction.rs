//! 連分数展開

/// $\frac{a}{b}$を連分数展開する。
///
/// `b`が`0`のとき、`None`を返す。
pub fn continued_fraction(mut a: u64, mut b: u64) -> Option<Vec<u64>> {
    if b == 0 {
        return None;
    }

    let mut ret = vec![];
    loop {
        ret.push(a / b);
        a %= b;
        if a == 0 {
            break;
        }
        std::mem::swap(&mut a, &mut b);
    }
    Some(ret)
}
