//! $\mathbb{F}_p$上の多項式
use std::ops::{Index, IndexMut};

use crate::math::ntt::NTT;
use crate::num::const_modint::*;

/// $\mathbb{F}_p$上の多項式
#[derive(Clone, Debug)]
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

    /// 内部の`Vec`の長さを返す。
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 項数が`0`のとき`true`を返す。
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 係数が`0`の高次項を縮める。
    pub fn shrink(&mut self) {
        while self.data.last().is_some_and(|x| x.value() == 0) {
            self.data.pop();
        }
    }

    /// [`len()`](Self::len())を超えないように、先頭`t`項をもつ多項式を返す。
    pub fn get_until(&self, t: usize) -> Self {
        Self {
            data: self.data[..t.min(self.len())].to_vec(),
        }
    }

    /// 多項式の次数を返す。
    ///
    /// `self`が零多項式のときは`None`を返す。
    ///
    /// **Time complexity** $O(n)$
    pub fn deg(&self) -> Option<usize> {
        (0..self.len()).rev().find(|&i| self.data[i].value() != 0)
    }
}

impl<const P: u32> PartialEq for Polynomial<P> {
    fn eq(&self, other: &Self) -> bool {
        let n = self.len().max(other.len());
        for i in 0..n {
            if self.coeff_of(i) != other.coeff_of(i) {
                return false;
            }
        }
        true
    }
}

impl<const P: u32> From<Polynomial<P>> for Vec<ConstModInt<P>> {
    fn from(value: Polynomial<P>) -> Self {
        value.data
    }
}

impl<T, const P: u32> From<Vec<T>> for Polynomial<P>
where
    T: Into<ConstModInt<P>>,
{
    fn from(value: Vec<T>) -> Self {
        Self {
            data: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<const P: u32> AsRef<[ConstModInt<P>]> for Polynomial<P> {
    fn as_ref(&self) -> &[ConstModInt<P>] {
        &self.data
    }
}

impl<const P: u32> AsMut<Vec<ConstModInt<P>>> for Polynomial<P> {
    fn as_mut(&mut self) -> &mut Vec<ConstModInt<P>> {
        &mut self.data
    }
}

impl<const P: u32> Index<usize> for Polynomial<P> {
    type Output = ConstModInt<P>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const P: u32> IndexMut<usize> for Polynomial<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

/// 多項式の演算を扱う。
pub struct PolynomialOperator<'a, const P: u32, const PR: u32> {
    pub(crate) ntt: &'a NTT<P, PR>,
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

    #[allow(missing_docs)]
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

    /// 多項式の微分を返す。
    pub fn differentiate(&self, a: Polynomial<P>) -> Polynomial<P> {
        let mut a: Vec<_> = a.into();
        let n = a.len();
        if n > 0 {
            for i in 0..n - 1 {
                a[i] = a[i + 1] * ConstModInt::new(i as u32 + 1);
            }
            a.pop();
        }
        a.into()
    }

    /// 多項式の積分を返す。
    pub fn integrate(&self, a: Polynomial<P>) -> Polynomial<P> {
        let mut a: Vec<_> = a.into();
        let n = a.len();
        let mut invs = vec![ConstModInt::new(1); n + 1];
        for i in 2..=n {
            invs[i] = -invs[P as usize % i] * ConstModInt::new(P / i as u32);
        }
        a.push(ConstModInt::new(0));
        for i in (0..n).rev() {
            a[i + 1] = a[i] * invs[i + 1];
        }
        a[0] = ConstModInt::new(0);

        a.into()
    }

    /// 係数を`k`次だけ高次側にずらす。ただし、$x^n$の項以降は無視する。
    ///
    /// $(a_0 + a_1 x + a_2 x^2 + \ldots + a_{n-1} x^{n-1}) \times x^k \pmod {x^n}$
    pub fn shift_higher(&self, a: Polynomial<P>, k: usize) -> Polynomial<P> {
        let a: Vec<_> = a.into();
        let n = a.len();
        let mut ret = vec![ConstModInt::new(0); n];

        ret[k..n].copy_from_slice(&a[..(n - k)]);

        ret.into()
    }

    /// 係数を`k`次だけ低次側にずらす。ただし、負の次数の項は無視する。
    pub fn shift_lower(&self, a: Polynomial<P>, k: usize) -> Polynomial<P> {
        let a: Vec<_> = a.into();
        let n = a.len();
        let mut ret = vec![ConstModInt::new(0); n];

        for i in (0..n.saturating_sub(k)).rev() {
            ret[i] = a[i + k];
        }

        ret.into()
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

    #[test]
    fn test_deg() {
        let check = |a: Vec<usize>, d: Option<usize>| {
            assert_eq!(Polynomial::<M>::from(a).deg(), d);
        };

        check(vec![1, 2, 3], Some(2));
        check(vec![1, 2, 3, 0, 0, 0], Some(2));
        check(vec![], None);
        check(vec![0, 0, 0, 0], None);
    }
}
