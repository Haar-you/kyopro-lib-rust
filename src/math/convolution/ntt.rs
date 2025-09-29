//! 数論変換 (Number Theoretic Transform)
use std::marker::PhantomData;

use crate::math::prime_mod::*;
use crate::num::const_modint::*;

/// 素数$P$上の数論変換 (Number Theoretic Transform)
///
/// `PRIM_ROOT`は`P`の原始根。
#[derive(Clone)]
pub struct NTT<P: PrimeMod> {
    _phantom: PhantomData<P>,
}

impl<P: PrimeMod> NTT<P> {
    const MAX_POWER: usize = (P::PRIME_NUM as usize - 1).trailing_zeros() as usize;
    const MAX_SIZE: usize = 1 << Self::MAX_POWER;
    const BASE: [ConstModInt<P>; 32] = {
        let mut base = [ConstModInt::<P>::new(0); 32];
        let mut t = ConstModInt::<P>::new(P::PRIM_ROOT)
            ._pow((P::PRIME_NUM as u64 - 1) >> (Self::MAX_POWER));

        let mut i = Self::MAX_POWER;
        while i > 0 {
            t = t._mul(t);
            base[i - 1] = t;
            i -= 1;
        }

        base
    };

    const INV_BASE: [ConstModInt<P>; 32] = {
        let mut inv_base = [ConstModInt::<P>::new(0); 32];
        let t = ConstModInt::<P>::new(P::PRIM_ROOT)
            ._pow((P::PRIME_NUM as u64 - 1) >> (Self::MAX_POWER));
        let mut s = t._inv();

        let mut i = Self::MAX_POWER;
        while i > 0 {
            s = s._mul(s);
            inv_base[i - 1] = s;
            i -= 1;
        }

        inv_base
    };

    /// [`NTT<P, PRIM_ROOT>`]を作る。
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    /// 数論変換を行う。
    pub fn ntt(&self, f: &mut [ConstModInt<P>]) {
        let n = f.len();
        assert!(n.is_power_of_two() && n < Self::MAX_SIZE);

        let mut b = n / 2;
        let mut k = n.trailing_zeros() as usize;
        while b > 0 {
            let dw = Self::BASE[k];

            let mut ws = vec![ConstModInt::new(1); b];
            for i in 1..b {
                ws[i] = ws[i - 1] * dw;
            }

            for a in f.chunks_exact_mut(2 * b) {
                let (x, y) = a.split_at_mut(b);

                for ((s, t), &w) in x.iter_mut().zip(y.iter_mut()).zip(ws.iter()) {
                    let p = *s + *t;
                    let q = (*s - *t) * w;

                    *s = p;
                    *t = q;
                }
            }

            k -= 1;
            b >>= 1;
        }

        // let p = size_of::<usize>() * 8 - n.trailing_zeros() as usize;
        // let mut g = vec![ConstModInt::new(0); n];
        // for i in 0..n {
        //     let j = i.reverse_bits() >> p;
        //     g[j] = f[i];
        // }
        // std::mem::swap(f, &mut g);
    }

    /// `ntt`の逆変換を行う。
    pub fn intt(&self, f: &mut [ConstModInt<P>]) {
        let n = f.len();
        assert!(n.is_power_of_two() && n < Self::MAX_SIZE);

        // let p = size_of::<usize>() * 8 - n.trailing_zeros() as usize;
        // let mut g = vec![ConstModInt::new(0); n];
        // for i in 0..n {
        //     let j = i.reverse_bits() >> p;
        //     g[j] = f[i];
        // }
        // std::mem::swap(f, &mut g);

        let mut b = 1;
        let mut k = 1;
        while b < n {
            let dw = Self::INV_BASE[k];

            let mut ws = vec![ConstModInt::new(1); b];
            for i in 1..b {
                ws[i] = ws[i - 1] * dw;
            }

            for a in f.chunks_exact_mut(2 * b) {
                let (x, y) = a.split_at_mut(b);

                for ((s, t), &w) in x.iter_mut().zip(y.iter_mut()).zip(ws.iter()) {
                    let p = *s + *t * w;
                    let q = *s - *t * w;

                    *s = p;
                    *t = q;
                }
            }

            k += 1;
            b <<= 1;
        }

        let t = ConstModInt::new(n as u32).inv();
        for x in f.iter_mut() {
            *x *= t;
        }
    }

    /// 2つの`Vec`を畳み込む。
    ///
    /// $(f \ast g)(k) = \sum_{k = i + j} f(i) \times g(j)$
    pub fn convolve(
        &self,
        mut f: Vec<ConstModInt<P>>,
        mut g: Vec<ConstModInt<P>>,
    ) -> Vec<ConstModInt<P>> {
        if f.is_empty() || g.is_empty() {
            return vec![];
        }

        let m = f.len() + g.len() - 1;
        let n = m.next_power_of_two();

        f.resize(n, ConstModInt::new(0));
        self.ntt(&mut f);

        g.resize(n, ConstModInt::new(0));
        self.ntt(&mut g);

        for (f, g) in f.iter_mut().zip(g.into_iter()) {
            *f *= g;
        }
        self.intt(&mut f);

        f
    }

    /// `convolve(f.clone(), f)`と同等。
    pub fn convolve_same(&self, mut f: Vec<ConstModInt<P>>) -> Vec<ConstModInt<P>> {
        if f.is_empty() {
            return vec![];
        }

        let n = (f.len() * 2 - 1).next_power_of_two();
        f.resize(n, ConstModInt::new(0));

        self.ntt(&mut f);

        for x in f.iter_mut() {
            *x *= *x;
        }

        self.intt(&mut f);
        f
    }

    /// NTTで変換可能な配列の最大長を返す。
    pub fn max_size(&self) -> usize {
        Self::MAX_SIZE
    }
}

impl<P: PrimeMod> Default for NTT<P> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        type P = Prime<998244353>;

        let ntt = NTT::<P>::new();
        let ff = ConstModIntBuilder::<P>::new();

        let mut rng = rand::thread_rng();

        let n = rng.gen_range(1..1000);
        let m = rng.gen_range(1..1000);

        let a = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..P::PRIME_NUM) as u64))
            .take(n)
            .collect::<Vec<_>>();
        let b = std::iter::repeat_with(|| ff.from_u64(rng.gen_range(0..P::PRIME_NUM) as u64))
            .take(m)
            .collect::<Vec<_>>();

        let res = ntt.convolve(a.clone(), b.clone());

        let mut ans = vec![ConstModInt::new(0); n + m - 1];

        for i in 0..n {
            for j in 0..m {
                ans[i + j] += a[i] * b[j];
            }
        }

        assert_eq!(&res[..n + m - 1], &ans);
    }
}
