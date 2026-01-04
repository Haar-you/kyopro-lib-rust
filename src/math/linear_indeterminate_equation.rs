//! 一次不定方程式$ax + by = c$
use crate::math::ext_gcd::ext_gcd;

/// 一次不定方程式$ax + by = c (a, b \neq 0)$を解く。
///
/// 方程式が解を持たないとき、`None`を返す。
///
/// 解を持つとき、`Some((x0, y0, s, t))`を返す。
/// $
/// \begin{cases}
/// x &= x_0 + s \times k \\\\
/// y &= y_0 + t \times k
/// \end{cases}
/// (k \in \mathbb{Z})$は方程式の一般解である。
/// $x_0, y_0$は解のうち$x$が非負で最小のものである。
pub fn linear_indeterminate_equation(a: i64, b: i64, c: i64) -> Option<(i64, i64, i64, i64)> {
    assert_ne!(a, 0);
    assert_ne!(b, 0);

    let (g, mut x, _) = ext_gcd(a.abs() as u64, b.abs() as u64);

    if c % g != 0 {
        return None;
    }

    let mut dx = b / g;
    let mut dy = -a / g;
    let dc = c / g;

    x %= dx;
    if x < 0 {
        x += dx.abs();
    }

    x *= dc;

    let y = (c - a * x) / b;

    if dx < 0 {
        dx = -dx;
        dy = -dy;
    }

    Some((x, y, dx, dy))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (a, b, c) = (111, 30, 12);
        let (x0, y0, s, t) = linear_indeterminate_equation(a, b, c).unwrap();

        for k in -100..=100 {
            let x = x0 + s * k;
            let y = y0 + t * k;
            assert_eq!(a * x + b * y, c);
        }
    }
}
