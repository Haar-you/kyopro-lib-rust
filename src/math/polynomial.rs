//! $\mathbb{F}_p$上の多項式
use crate::math::ntt::NTT;
use crate::num::const_modint::*;

/// $\mathbb{F}_p$上の多項式
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial<const P: u32> {
    data: Vec<ConstModInt<P>>,
}

impl<const P: u32> Polynomial<P> {
    /// 零多項式を得る。
    pub fn zero() -> Self {
        Self { data: vec![] }
    }

    /// 定数項のみをもつ多項式を生成する。
    pub fn constant(a: ConstModInt<P>) -> Self {
        if a.value() == 0 {
            Self::zero()
        } else {
            Self { data: vec![a] }
        }
    }

    /// $x^i$の係数を得る。
    pub fn coeff_of(&self, i: usize) -> ConstModInt<P> {
        self.data.get(i).map_or(ConstModInt::new(0), |a| *a)
    }

    /// 多項式に値`p`を代入した結果を求める。
    pub fn eval(&self, p: ConstModInt<P>) -> ConstModInt<P> {
        let mut ret = ConstModInt::new(0);
        let mut x = ConstModInt::new(1);

        for &a in &self.data {
            ret += a * x;
            x *= p;
        }

        ret
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn shrink(&mut self) {
        while self.data.last().is_some_and(|x| x.value() == 0) {
            self.data.pop();
        }
    }

    pub fn get_until(&self, t: usize) -> Self {
        Self {
            data: self.data[..t.min(self.len())].to_vec(),
        }
    }

    /// 多項式の次数を返す。
    ///
    /// `self`が零多項式のときは`None`を返す。
    pub fn deg(&self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }
        if self.len() == 1 && self.data[0].value() == 0 {
            return None;
        }
        Some(self.len() - 1)
    }
}

impl<const P: u32> From<Vec<ConstModInt<P>>> for Polynomial<P> {
    fn from(data: Vec<ConstModInt<P>>) -> Self {
        let mut this = Self { data };
        this.shrink();
        this
    }
}

/// 多項式の演算を扱う。
pub struct PolynomialOperator<'a, const P: u32, const PR: u32> {
    ntt: &'a NTT<P, PR>,
}

impl<'a, const P: u32, const PR: u32> PolynomialOperator<'a, P, PR> {
    /// [`NTT<P>`]を基に`PolynomialOperator<P>`を生成する。
    pub fn new(ntt: &'a NTT<P, PR>) -> Self {
        Self { ntt }
    }

    /// 多項式`a`に多項式`b`を足す。
    pub fn add_assign(&self, a: &mut Polynomial<P>, b: Polynomial<P>) {
        if a.len() < b.len() {
            a.data.resize(b.len(), ConstModInt::new(0));
        }
        for (a, b) in a.data.iter_mut().zip(b.data.into_iter()) {
            *a += b;
        }
    }

    /// 多項式`a`と多項式`b`の和を返す。
    pub fn add(&self, mut a: Polynomial<P>, b: Polynomial<P>) -> Polynomial<P> {
        self.add_assign(&mut a, b);
        a
    }

    /// 多項式`a`から多項式`b`を引く。
    pub fn sub_assign(&self, a: &mut Polynomial<P>, b: Polynomial<P>) {
        if a.len() < b.len() {
            a.data.resize(b.len(), ConstModInt::new(0));
        }
        for (a, b) in a.data.iter_mut().zip(b.data.into_iter()) {
            *a -= b;
        }
    }

    /// 多項式`a`と多項式`b`の差を返す。
    pub fn sub(&self, mut a: Polynomial<P>, b: Polynomial<P>) -> Polynomial<P> {
        self.sub_assign(&mut a, b);
        a
    }

    /// 多項式`a`に多項式`b`を掛ける。
    pub fn mul_assign(&self, a: &mut Polynomial<P>, b: Polynomial<P>) {
        let k = a.len() + b.len() - 1;
        a.data = self.ntt.convolve(a.data.clone(), b.data);
        a.data.truncate(k);
    }

    /// 多項式`a`と多項式`b`の積を返す。
    pub fn mul(&self, mut a: Polynomial<P>, b: Polynomial<P>) -> Polynomial<P> {
        self.mul_assign(&mut a, b);
        a
    }

    /// 多項式`a`の2乗を返す。
    pub fn sq(&self, a: Polynomial<P>) -> Polynomial<P> {
        self.mul(a.clone(), a)
    }

    /// 多項式`a`の`k`倍を返す。
    pub fn scale(&self, a: Polynomial<P>, k: ConstModInt<P>) -> Polynomial<P> {
        Polynomial {
            data: a.data.into_iter().map(|x| x * k).collect(),
        }
    }

    pub fn inv(&self, a: Polynomial<P>, n: usize) -> Polynomial<P> {
        let mut ret = Polynomial::constant(a.data[0].inv());
        let mut t = 1;

        while t <= n * 2 {
            ret = self.sub(
                self.scale(ret.clone(), ConstModInt::new(2)),
                self.mul(self.sq(ret).get_until(t), a.clone().get_until(t)),
            );
            ret.data.truncate(t);
            t *= 2;
        }

        ret
    }

    /// 多項式`a`の多項式`b`による商と剰余を返す。
    pub fn divmod(&self, a: Polynomial<P>, b: Polynomial<P>) -> (Polynomial<P>, Polynomial<P>) {
        if a.len() < b.len() {
            return (Polynomial::zero(), a);
        }

        let m = a.len() - b.len();

        let mut g = a.clone();
        g.data.reverse();

        let mut f = b.clone();
        f.data.reverse();

        f = self.inv(f, m);
        f.data.resize(m + 1, ConstModInt::new(0));

        let mut q = self.mul(f, g);
        q.data.resize(m + 1, ConstModInt::new(0));
        q.data.reverse();

        let d = b.len() - 1;
        let mut r = self.sub(a, self.mul(b, q.clone()));
        r.data.truncate(d);

        r.shrink();
        q.shrink();

        (q, r)
    }
}

#[cfg(test)]
mod tests {
    use crate::num::const_modint::ConstModIntBuilder;

    use super::*;

    const M: u32 = 998244353;

    #[test]
    fn test() {
        let ff = ConstModIntBuilder::<M>;
        let ntt = NTT::<M, 3>::new();
        let po = PolynomialOperator::new(&ntt);

        let a: Vec<_> = vec![5, 4, 3, 2, 1]
            .into_iter()
            .map(|x| ff.from_u64(x))
            .collect();
        let a = Polynomial::from(a);

        let b: Vec<_> = vec![1, 2, 3, 4, 5]
            .into_iter()
            .map(|x| ff.from_u64(x))
            .collect();
        let b = Polynomial::from(b);

        let (q, r) = po.divmod(a.clone(), b.clone());

        let a_ = po.add(po.mul(q, b.clone()), r);
        assert_eq!(a, a_);
    }
}
