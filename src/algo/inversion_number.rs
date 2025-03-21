//! 転倒数

/// 数列の転倒数を計算する。
///
/// **Time complexity** $O(n \log n)$
pub fn inversion_number<T: PartialOrd + Copy>(a: &mut [T]) -> u64 {
    let n = a.len();

    if n <= 1 {
        return 0;
    }

    let mut ret = 0;

    let b = &mut a[0..n / 2].to_vec();
    let c = &mut a[n / 2..n].to_vec();

    ret += inversion_number(b);
    ret += inversion_number(c);

    let mut bi = 0;
    let mut ci = 0;

    for ai in a.iter_mut() {
        if bi < b.len() && (ci == c.len() || b[bi] <= c[ci]) {
            *ai = b[bi];
            bi += 1;
        } else {
            ret += (n / 2 - bi) as u64;
            *ai = c[ci];
            ci += 1;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(inversion_number(&mut [3, 5, 2, 1, 4]), 6);
        assert_eq!(inversion_number(&mut [3, 1, 2]), 2);
    }
}
