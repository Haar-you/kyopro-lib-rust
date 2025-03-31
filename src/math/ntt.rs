//! 数論変換 (Number Theoretic Transform)
use crate::num::const_modint::*;

/// 素数$P$上の数論変換 (Number Theoretic Transform)
///
/// `PRIM_ROOT`は`P`の原始根。
pub struct NTT<const P: u32, const PRIM_ROOT: u32> {
    base: Vec<ConstModInt<P>>,
    inv_base: Vec<ConstModInt<P>>,
    max_size: usize,
}

impl<const P: u32, const PRIM_ROOT: u32> NTT<P, PRIM_ROOT> {
    /// [`NTT<P, PRIM_ROOT>`]を作る。
    pub fn new() -> Self {
        let max_power = (P as usize - 1).trailing_zeros() as usize;
        let max_size = 1 << max_power;

        let mut base = vec![ConstModInt::new(0); max_power + 1];
        let mut inv_base = vec![ConstModInt::new(0); max_power + 1];

        let mut t = ConstModInt::new(PRIM_ROOT).pow((P as u64 - 1) >> (max_power));
        let mut s = t.inv();

        for i in (0..max_power).rev() {
            t *= t;
            s *= s;
            base[i] = t;
            inv_base[i] = s;
        }

        Self {
            base,
            inv_base,
            max_size,
        }
    }

    fn run(&self, f: &mut Vec<ConstModInt<P>>, inv: bool) {
        let n = f.len();
        assert!(n.is_power_of_two() && n < self.max_size);

        let mut g = vec![ConstModInt::new(0); n];

        let mut b = n >> 1;
        let mut k = 1;
        while b > 0 {
            let dw = if !inv { self.base[k] } else { self.inv_base[k] };
            let len = n / b;

            let mut w = ConstModInt::new(1);

            for j in 0..len / 2 {
                for i in 0..b {
                    let even = unsafe { *f.get_unchecked(i + 2 * j * b) };
                    let odd = unsafe { *f.get_unchecked(i + 2 * j * b + b) };

                    unsafe {
                        *g.get_unchecked_mut(i + j * b) = even + w * odd;
                        *g.get_unchecked_mut(i + j * b + n / 2) = even - w * odd;
                    }
                }

                w *= dw;
            }

            k += 1;
            b >>= 1;

            std::mem::swap(&mut g, f);
        }

        if inv {
            let t = ConstModInt::new(n as u32).inv();
            for x in f.iter_mut() {
                *x *= t;
            }
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
        self.run(&mut f, false);

        g.resize(n, ConstModInt::new(0));
        self.run(&mut g, false);

        for (f, g) in f.iter_mut().zip(g.into_iter()) {
            *f *= g;
        }
        self.run(&mut f, true);

        f
    }

    /// `convolve(f.clone(), f)`と同等。
    pub fn convolve_same(&self, mut f: Vec<ConstModInt<P>>) -> Vec<ConstModInt<P>> {
        if f.is_empty() {
            return vec![];
        }

        let n = (f.len() * 2 - 1).next_power_of_two();
        f.resize(n, ConstModInt::new(0));

        self.run(&mut f, false);

        for x in f.iter_mut() {
            *x *= *x;
        }

        self.run(&mut f, true);
        f
    }

    /// NTTで変換可能な配列の最大長を返す。
    pub fn max_size(&self) -> usize {
        self.max_size
    }
}

/// $\mod 998244353 (= 2^{23} * 7 * 17 + 1)$上の`NTT`
pub type NTT998244353 = NTT<998244353, 3>;

#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        const MOD: u32 = 998244353;

        let ntt = NTT998244353::new();
        let ff = ConstModIntBuilder::<MOD>;

        let mut rng = rand::thread_rng();

        let n = rng.gen_range(1..1000);
        let m = rng.gen_range(1..1000);

        let a = (0..n)
            .map(|_| ff.from_u64(rng.gen_range(0..MOD) as u64))
            .collect::<Vec<_>>();
        let b = (0..m)
            .map(|_| ff.from_u64(rng.gen_range(0..MOD) as u64))
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
