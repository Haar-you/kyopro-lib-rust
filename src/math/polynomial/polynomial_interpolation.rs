//! 多項式補間
use crate::math::polynomial::{multipoint_eval::MultipointEval, Polynomial};
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::ConstModInt;

/// $y_0 = f(x_0), \dots, y_{n-1} = f(x_{n-1})$を満たす多項式$f(x) = c_0 x^0 + c_1 x^1 + \dots + c_{n-1} x^{n-1}$を求める。
pub fn polynomial_interpolation<P: PrimeMod>(
    xs: Vec<impl Into<ConstModInt<P>>>,
    ys: Vec<impl Into<ConstModInt<P>>>,
) -> Polynomial<P> {
    assert_eq!(xs.len(), ys.len());

    let n = xs.len();
    let xs = xs.into_iter().map(Into::into).collect::<Vec<_>>();
    let ys = ys.into_iter().map(Into::into).collect::<Vec<_>>();

    let g = rec_g(0, n, &xs);

    let mut gd = g.clone();
    gd.differentiate();
    let gd = gd.multipoint_eval(xs.clone());

    let (a, b) = rec_frac(0, n, &xs, &ys, &gd);

    let t = a * g;
    t / b
}

fn rec_g<P: PrimeMod>(l: usize, r: usize, xs: &[ConstModInt<P>]) -> Polynomial<P> {
    if r - l == 1 {
        return vec![-xs[l], 1.into()].into();
    }

    let m = (l + r) / 2;
    rec_g(l, m, xs) * rec_g(m, r, xs)
}

fn rec_frac<P: PrimeMod>(
    l: usize,
    r: usize,
    xs: &[ConstModInt<P>],
    ys: &[ConstModInt<P>],
    gs: &[ConstModInt<P>],
) -> (Polynomial<P>, Polynomial<P>) {
    if r - l == 1 {
        return (vec![ys[l]].into(), vec![-xs[l] * gs[l], gs[l]].into());
    }

    let m = (l + r) / 2;

    let (la, lb) = rec_frac(l, m, xs, ys, gs);
    let (ra, rb) = rec_frac(m, r, xs, ys, gs);

    let deno = lb.clone() * rb.clone();
    let nume = la * rb + ra * lb;

    (nume, deno)
}

#[cfg(test)]
mod tests {
    use crate::math::prime_mod::Prime;

    use super::*;

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let xs = vec![5, 6, 7, 8, 9];
        let ys = vec![586, 985, 1534, 2257, 3178];

        let p = polynomial_interpolation::<P>(xs, ys);

        assert_eq!(p, Polynomial::from(vec![1, 2, 3, 4, 0]));
    }
}
