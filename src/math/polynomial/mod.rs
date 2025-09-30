//! 多項式

pub mod multipoint_eval;
pub mod polynomial_interpolation;
pub mod polynomial_taylor_shift;
pub mod shift_sampling_points;
pub mod sparse;

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};

use crate::math::convolution::ntt::NTT;
use crate::math::prime_mod::PrimeMod;
use crate::num::const_modint::*;

/// $\mathbb{F}_p$上の多項式
#[derive(Clone, Debug, Default)]
pub struct Polynomial<P: PrimeMod> {
    pub(crate) data: Vec<ConstModInt<P>>,
}

impl<P: PrimeMod> Polynomial<P> {
    pub(crate) const NTT: NTT<P> = NTT::<P>::new();

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
            invs[i] = -invs[P::PRIME_NUM as usize % i] * ConstModInt::new(P::PRIME_NUM / i as u32);
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

    /// 多項式の列の積を計算する。
    pub fn prod(mut a: Vec<Self>) -> Self {
        match a.len() {
            0 => Self::constant(1.into()),
            1 => a.pop().unwrap(),
            n => {
                let b = a.split_off(n / 2);
                Self::prod(a) * Self::prod(b)
            }
        }
    }

    /// 多項式`a`の2乗を返す。
    pub fn sq(mut self) -> Self {
        let k = self.len() * 2 - 1;
        let n = k.next_power_of_two();

        self.data.resize(n, 0.into());
        Self::NTT.ntt(&mut self.data);
        self.data.iter_mut().for_each(|x| *x *= *x);
        Self::NTT.intt(&mut self.data);

        self.data.truncate(k);
        self
    }

    #[allow(missing_docs)]
    pub fn inv(self, n: usize) -> Self {
        let mut t = 1;
        let mut ret = vec![self.data[0].inv()];
        let a: Vec<_> = self.into();

        while t <= n * 2 {
            let k = (t * 2 - 1).next_power_of_two();

            let mut s = ret.clone();
            s.resize(k, 0.into());
            Self::NTT.ntt(&mut s);
            s.iter_mut().for_each(|x| *x *= *x);

            let mut a = a[..t.min(a.len())].to_vec();
            a.resize(k, 0.into());
            Self::NTT.ntt(&mut a);

            s.iter_mut().zip(a).for_each(|(x, y)| *x *= y);
            Self::NTT.intt(&mut s);

            ret.resize(t, 0.into());
            ret.iter_mut()
                .zip(s)
                .for_each(|(x, y)| *x = *x * 2.into() - y);

            t *= 2;
        }

        ret.into()
    }

    /// 多項式`a`の多項式`b`による商と剰余を返す。
    pub fn divrem(self, b: Self) -> (Self, Self) {
        if self.len() < b.len() {
            return (Self::zero(), self);
        }

        let q = self.clone() / b.clone();

        let d = b.len() - 1;
        let mut r = self - b * q.clone();
        r.data.truncate(d);
        r.shrink();

        (q, r)
    }
}

impl<P: PrimeMod> AddAssign for Polynomial<P> {
    fn add_assign(&mut self, b: Self) {
        if self.len() < b.len() {
            self.data.resize(b.len(), ConstModInt::new(0));
        }
        for (a, b) in self.data.iter_mut().zip(b.data) {
            *a += b;
        }
    }
}

impl<P: PrimeMod> Add for Polynomial<P> {
    type Output = Self;
    fn add(mut self, b: Self) -> Self {
        self += b;
        self
    }
}

impl<P: PrimeMod> SubAssign for Polynomial<P> {
    fn sub_assign(&mut self, b: Self) {
        if self.len() < b.len() {
            self.data.resize(b.len(), ConstModInt::new(0));
        }
        for (a, b) in self.data.iter_mut().zip(b.data) {
            *a -= b;
        }
    }
}

impl<P: PrimeMod> Sub for Polynomial<P> {
    type Output = Self;
    fn sub(mut self, b: Self) -> Self {
        self -= b;
        self
    }
}

impl<P: PrimeMod> MulAssign for Polynomial<P> {
    fn mul_assign(&mut self, mut rhs: Self) {
        let k = self.len() + rhs.len() - 1;

        let n = k.next_power_of_two();
        self.data.resize(n, 0.into());
        Self::NTT.ntt(&mut self.data);

        rhs.data.resize(n, 0.into());
        Self::NTT.ntt(&mut rhs.data);

        self.data
            .iter_mut()
            .zip(rhs.data)
            .for_each(|(x, y)| *x *= y);
        Self::NTT.intt(&mut self.data);

        self.data.truncate(k);
    }
}

impl<P: PrimeMod> Mul for Polynomial<P> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<P: PrimeMod> DivAssign for Polynomial<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<P: PrimeMod> Div for Polynomial<P> {
    type Output = Self;
    fn div(mut self, mut rhs: Self) -> Self::Output {
        if self.len() < rhs.len() {
            return Self::zero();
        }

        let m = self.len() - rhs.len();

        self.data.reverse();
        rhs.data.reverse();

        rhs = rhs.inv(m);
        rhs.data.resize(m + 1, 0.into());

        let mut q = self * rhs;
        q.data.resize(m + 1, 0.into());
        q.data.reverse();
        q.shrink();
        q
    }
}

impl<P: PrimeMod> RemAssign for Polynomial<P> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone() % rhs;
    }
}

impl<P: PrimeMod> Rem for Polynomial<P> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        self.divrem(rhs).1
    }
}

impl<P: PrimeMod> PartialEq for Polynomial<P> {
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

impl<P: PrimeMod> Eq for Polynomial<P> {}

impl<P: PrimeMod> From<Polynomial<P>> for Vec<ConstModInt<P>> {
    fn from(value: Polynomial<P>) -> Self {
        value.data
    }
}

impl<T, P: PrimeMod> From<Vec<T>> for Polynomial<P>
where
    T: Into<ConstModInt<P>>,
{
    fn from(value: Vec<T>) -> Self {
        Self {
            data: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl<P: PrimeMod> AsRef<[ConstModInt<P>]> for Polynomial<P> {
    fn as_ref(&self) -> &[ConstModInt<P>] {
        &self.data
    }
}

impl<P: PrimeMod> AsMut<Vec<ConstModInt<P>>> for Polynomial<P> {
    fn as_mut(&mut self) -> &mut Vec<ConstModInt<P>> {
        &mut self.data
    }
}

impl<P: PrimeMod> Index<usize> for Polynomial<P> {
    type Output = ConstModInt<P>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<P: PrimeMod> IndexMut<usize> for Polynomial<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{math::prime_mod::Prime, num::const_modint::ConstModIntBuilder};

    use super::*;

    type P = Prime<998244353>;

    #[test]
    fn test() {
        let ff = ConstModIntBuilder::<P>::new();

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

        let (q, r) = a.clone().divrem(b.clone());

        let a_ = q * b.clone() + r;
        assert_eq!(a, a_);
    }

    #[test]
    fn test_deg() {
        let check = |a: Vec<usize>, d: Option<usize>| {
            assert_eq!(Polynomial::<P>::from(a).deg(), d);
        };

        check(vec![1, 2, 3], Some(2));
        check(vec![1, 2, 3, 0, 0, 0], Some(2));
        check(vec![], None);
        check(vec![0, 0, 0, 0], None);
    }
}
