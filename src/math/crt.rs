use crate::math::ext_gcd::ext_gcd;

pub fn crt((b1, m1): (i64, u64), (b2, m2): (i64, u64)) -> Option<(i64, u64)> {
    let (d, p, _) = ext_gcd(m1, m2);

    let m1 = m1 as i64;
    let m2 = m2 as i64;

    if (b2 - b1) % d != 0 {
        return None;
    }

    let m = m1 / d * m2;
    let t = ((b2 - b1) * p / d) % (m2 / d);
    let r = (b1 + m1 * t + m) % m;

    Some((r, m as u64))
}

pub fn crt_vec(params: &[(i64, u64)]) -> Option<(i64, u64)> {
    let bs = params.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let ms = params.iter().map(|(_, y)| *y).collect::<Vec<_>>();
    let n = params.len();

    let mut _r = 0;
    let mut _m = 1;

    for i in 0 .. n {
        if let Some((r, m)) = crt((_r, _m), (bs[i], ms[i])) {
            _r = r;
            _m = m;
        }
        else {
            return None;
        }
    }

    Some((_r, _m))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(crt((2, 3), (3, 5)), Some((8, 15)));

        // https://yukicoder.me/problems/447
        assert_eq!(crt_vec(&[(10, 20), (10, 30), (30, 40)]), Some((70, 120)));
        assert_eq!(crt_vec(&[(1, 2), (0, 4), (5, 17)]), None);
    }
}
