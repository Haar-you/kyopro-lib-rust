use crate::math::multipoint_eval::MultipointEval;
use crate::math::ntt::*;
use crate::math::polynomial::{Polynomial, PolynomialOperator};
use crate::num::const_modint::ConstModInt;

pub fn polynomial_interpolation<const P: u32, const PR: u32>(
    xs: Vec<impl Into<ConstModInt<P>>>,
    ys: Vec<impl Into<ConstModInt<P>>>,
    ntt: &NTT<P, PR>,
) -> Polynomial<P> {
    assert_eq!(xs.len(), ys.len());
    let n = xs.len();
    let xs = xs.into_iter().map(Into::into).collect::<Vec<_>>();
    let ys = ys.into_iter().map(Into::into).collect::<Vec<_>>();

    let po = PolynomialOperator::new(ntt);

    let g = rec_g(0, n, &xs, &ys, ntt);

    let gd = po.differentiate(g.clone());
    let gd = po.multipoint_eval(gd, xs.clone());

    let (a, b) = rec_frac(0, n, &xs, &ys, &gd, ntt);

    let t = po.mul(a, g);
    po.divmod(t, b).0
}

fn rec_g<const P: u32, const PR: u32>(
    l: usize,
    r: usize,
    xs: &[ConstModInt<P>],
    ys: &[ConstModInt<P>],
    ntt: &NTT<P, PR>,
) -> Polynomial<P> {
    if r - l == 1 {
        return vec![-xs[l], 1.into()].into();
    }

    let po = PolynomialOperator::new(ntt);
    let m = (l + r) / 2;
    po.mul(rec_g(l, m, xs, ys, ntt), rec_g(m, r, xs, ys, ntt))
}

fn rec_frac<const P: u32, const PR: u32>(
    l: usize,
    r: usize,
    xs: &[ConstModInt<P>],
    ys: &[ConstModInt<P>],
    gs: &[ConstModInt<P>],
    ntt: &NTT<P, PR>,
) -> (Polynomial<P>, Polynomial<P>) {
    if r - l == 1 {
        return (vec![ys[l]].into(), vec![-xs[l] * gs[l], gs[l]].into());
    }

    let m = (l + r) / 2;

    let (la, lb) = rec_frac(l, m, xs, ys, gs, ntt);
    let (ra, rb) = rec_frac(m, r, xs, ys, gs, ntt);

    let po = PolynomialOperator::new(&ntt);
    let deno = po.mul(lb.clone(), rb.clone());
    let nume = po.add(po.mul(la, rb), po.mul(ra, lb));

    (nume, deno)
}
