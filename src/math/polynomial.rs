//! $\mathbb{F}_p$上の多項式
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

use crate::math::ntt::NTT;
use crate::num::const_modint::*;

/// $\mathbb{F}_p$上の多項式
#[derive(Clone, Debug, Default)]
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

    /// 多項式を`k`倍する。
    pub fn scale(&mut self, k: ConstModInt<P>) {
        self.data.iter_mut().for_each(|x| *x *= k);
    }

    /// 多項式を微分する。
    pub fn differentiate(&mut self) {
        let n = self.len();
        if n > 0 {
            for i in 0..n - 1 {
                self.data[i] = self.data[i + 1] * ConstModInt::new(i as u32 + 1);
            }
            self.data.pop();
        }
    }

    /// 多項式を積分する。
    pub fn integrate(&mut self) {
        let n = self.len();
        let mut invs = vec![ConstModInt::new(1); n + 1];
        for i in 2..=n {
            invs[i] = -invs[P as usize % i] * ConstModInt::new(P / i as u32);
        }
        self.data.push(0.into());
        for i in (0..n).rev() {
            self.data[i + 1] = self.data[i] * invs[i + 1];
        }
        self.data[0] = 0.into();
    }

    /// 係数を`k`次だけ高次側にずらす。ただし、$x^n$の項以降は無視する。
    ///
    /// $(a_0 + a_1 x + a_2 x^2 + \ldots + a_{n-1} x^{n-1}) \times x^k \pmod {x^n}$
    pub fn shift_higher(&mut self, k: usize) {
        let n = self.len();
        for i in (k..n).rev() {
            self.data[i] = self.data[i - k];
        }
        for i in 0..k {
            self.data[i] = 0.into();
        }
    }

    /// 係数を`k`次だけ低次側にずらす。ただし、負の次数の項は無視する。
    pub fn shift_lower(&mut self, k: usize) {
        let n = self.len();
        for i in 0..n.saturating_sub(k) {
            self.data[i] = self.data[i + k];
        }
        for i in n.saturating_sub(k)..n {
            self.data[i] = 0.into();
        }
    }
}

impl<const P: u32> AddAssign for Polynomial<P> {
    fn add_assign(&mut self, b: Polynomial<P>) {
        if self.len() < b.len() {
            self.data.resize(b.len(), ConstModInt::new(0));
        }
        for (a, b) in self.data.iter_mut().zip(b.data) {
            *a += b;
        }
    }
}

impl<const P: u32> Add for Polynomial<P> {
    type Output = Self;
    fn add(mut self, b: Polynomial<P>) -> Polynomial<P> {
        self += b;
        self
    }
}

impl<const P: u32> SubAssign for Polynomial<P> {
    fn sub_assign(&mut self, b: Polynomial<P>) {
        if self.len() < b.len() {
            self.data.resize(b.len(), ConstModInt::new(0));
        }
        for (a, b) in self.data.iter_mut().zip(b.data) {
            *a -= b;
        }
    }
}

impl<const P: u32> Sub for Polynomial<P> {
    type Output = Self;
    fn sub(mut self, b: Polynomial<P>) -> Polynomial<P> {
        self -= b;
        self
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

    /// 多項式`a`に多項式`b`を掛ける。
    pub fn mul_assign(&self, a: &mut Polynomial<P>, mut b: Polynomial<P>) {
        let k = a.len() + b.len() - 1;

        let n = k.next_power_of_two();
        a.data.resize(n, 0.into());
        self.ntt.ntt(&mut a.data);

        b.data.resize(n, 0.into());
        self.ntt.ntt(&mut b.data);

        a.data.iter_mut().zip(b.data).for_each(|(x, y)| *x *= y);
        self.ntt.intt(&mut a.data);

        a.data.truncate(k);
    }

    /// 多項式`a`と多項式`b`の積を返す。
    pub fn mul(&self, mut a: Polynomial<P>, b: Polynomial<P>) -> Polynomial<P> {
        self.mul_assign(&mut a, b);
        a
    }

    /// 多項式`a`の2乗を返す。
    pub fn sq(&self, mut a: Polynomial<P>) -> Polynomial<P> {
        let k = a.len() * 2 - 1;
        let n = k.next_power_of_two();

        a.data.resize(n, 0.into());
        self.ntt.ntt(&mut a.data);
        a.data.iter_mut().for_each(|x| *x *= *x);
        self.ntt.intt(&mut a.data);

        a.data.truncate(k);
        a
    }

    #[allow(missing_docs)]
    pub fn inv(&self, a: Polynomial<P>, n: usize) -> Polynomial<P> {
        let mut t = 1;
        let mut ret = vec![a.data[0].inv()];
        let a: Vec<_> = a.into();

        while t <= n * 2 {
            let k = (t * 2 - 1).next_power_of_two();

            let mut s = ret.clone();
            s.resize(k, 0.into());
            self.ntt.ntt(&mut s);
            s.iter_mut().for_each(|x| *x *= *x);

            let mut a = a[..t.min(a.len())].to_vec();
            a.resize(k, 0.into());
            self.ntt.ntt(&mut a);

            s.iter_mut().zip(a).for_each(|(x, y)| *x *= y);
            self.ntt.intt(&mut s);

            ret.resize(t, 0.into());
            ret.iter_mut()
                .zip(s)
                .for_each(|(x, y)| *x = *x * 2.into() - y);

            t *= 2;
        }

        ret.into()
    }

    /// 多項式`a`の多項式`b`による商を返す。
    pub fn div(&self, mut a: Polynomial<P>, mut b: Polynomial<P>) -> Polynomial<P> {
        if a.len() < b.len() {
            return Polynomial::zero();
        }

        let m = a.len() - b.len();

        a.data.reverse();
        b.data.reverse();

        b = self.inv(b, m);
        b.data.resize(m + 1, 0.into());

        let mut q = self.mul(a, b);
        q.data.resize(m + 1, 0.into());
        q.data.reverse();
        q.shrink();
        q
    }

    /// 多項式`a`の多項式`b`による剰余を返す。
    pub fn rem(&self, a: Polynomial<P>, b: Polynomial<P>) -> Polynomial<P> {
        self.divrem(a, b).1
    }

    /// 多項式`a`の多項式`b`による商と剰余を返す。
    pub fn divrem(&self, a: Polynomial<P>, b: Polynomial<P>) -> (Polynomial<P>, Polynomial<P>) {
        if a.len() < b.len() {
            return (Polynomial::zero(), a);
        }

        let q = self.div(a.clone(), b.clone());

        let d = b.len() - 1;
        let mut r = a.sub(self.mul(b, q.clone()));
        r.data.truncate(d);
        r.shrink();

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

        let (q, r) = po.divrem(a.clone(), b.clone());

        let a_ = po.mul(q, b.clone()) + r;
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
