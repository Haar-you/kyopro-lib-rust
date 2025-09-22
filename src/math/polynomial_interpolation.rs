//! 多項式補間
use crate::math::multipoint_eval::MultipointEval;
use crate::math::polynomial::{Polynomial, PolynomialOperator};
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

    let po = PolynomialOperator::<P>::new();

    let g = rec_g(0, n, &xs, &po);

    let mut gd = g.clone();
    gd.differentiate();
    let gd = po.multipoint_eval(gd, xs.clone());

    let (a, b) = rec_frac(0, n, &xs, &ys, &gd, &po);

    let t = po.mul(a, g);
    po.div(t, b)
}

fn rec_g<P: PrimeMod>(
    l: usize,
    r: usize,
    xs: &[ConstModInt<P>],
    po: &PolynomialOperator<P>,
) -> Polynomial<P> {
    if r - l == 1 {
        return vec![-xs[l], 1.into()].into();
    }

    let m = (l + r) / 2;
    po.mul(rec_g(l, m, xs, po), rec_g(m, r, xs, po))
}

fn rec_frac<P: PrimeMod>(
    l: usize,
    r: usize,
    xs: &[ConstModInt<P>],
    ys: &[ConstModInt<P>],
    gs: &[ConstModInt<P>],
    po: &PolynomialOperator<P>,
) -> (Polynomial<P>, Polynomial<P>) {
    if r - l == 1 {
        return (vec![ys[l]].into(), vec![-xs[l] * gs[l], gs[l]].into());
    }

    let m = (l + r) / 2;

    let (la, lb) = rec_frac(l, m, xs, ys, gs, po);
    let (ra, rb) = rec_frac(m, r, xs, ys, gs, po);

    let deno = po.mul(lb.clone(), rb.clone());
    let nume = po.mul(la, rb) + po.mul(ra, lb);

    (nume, deno)
}
