/// 数列の転倒数を計算する。
///
/// Time complexity O(n log(n))
pub fn inversion_number<T: Clone + PartialOrd + Copy>(a: &mut [T]) -> u64 {
    let n = a.len();

    if n <= 1 {
        return 0;
    }

    let mut ret = 0;

    let mut b = &mut a[0 .. n / 2].to_vec();
    let mut c = &mut a[n / 2 .. n].to_vec();

    ret += inversion_number(&mut b);
    ret += inversion_number(&mut c);

    let mut ai = 0;
    let mut bi = 0;
    let mut ci = 0;

    while ai < n {
        if bi < b.len() && (ci == c.len() || !(b[bi] > c[ci])) {
            a[ai] = b[bi];
            bi += 1;
        }
        else {
            ret += (n / 2 - bi) as u64;
            a[ai] = c[ci];
            ci += 1;
        }
        ai += 1;
    }

    ret
}
