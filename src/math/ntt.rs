use super::ff::const_modint::ConstModInt;
use crate::math::ff::modint::{Inv, Pow};

pub struct NTT<const P: u32, const PRIM_ROOT: u32> {
    base: Vec<ConstModInt<P>>,
    inv_base: Vec<ConstModInt<P>>,
    max_size: usize,
}

impl<const P: u32, const PRIM_ROOT: u32> NTT<P, PRIM_ROOT> {
    pub fn new(max_size: usize) -> Self {
        assert!(max_size.is_power_of_two());
        assert!((P as usize - 1) % max_size == 0);

        let max_power = max_size.trailing_zeros() as usize;

        let mut base = vec![ConstModInt::new(0); max_power + 1];
        let mut inv_base = vec![ConstModInt::new(0); max_power + 1];

        let mut t = ConstModInt::new(PRIM_ROOT).pow((P as u64 - 1) >> (max_power + 2));
        let mut s = t.inv();

        for i in (0..max_power).rev() {
            t *= t;
            s *= s;
            base[i] = -t;
            inv_base[i] = -s;
        }

        Self {
            base,
            inv_base,
            max_size,
        }
    }

    fn run(&self, f: &mut [ConstModInt<P>]) {
        let n = f.len();
        assert!(n.is_power_of_two() && n <= self.max_size);

        let mut b = n >> 1;
        while b > 0 {
            let mut w = ConstModInt::new(1);

            let mut j = 0;
            let mut k: u32 = 1;
            while j < n {
                for i in 0..b {
                    let s = f[i + j];
                    let t = f[i + j + b] * w;

                    f[i + j] = s + t;
                    f[i + j + b] = s - t;
                }
                w *= self.base[k.trailing_zeros() as usize];
                j += 2 * b;
                k += 1;
            }

            b >>= 1;
        }
    }

    fn run_inv(&self, f: &mut [ConstModInt<P>]) {
        let n = f.len();
        assert!(n.is_power_of_two() && n <= self.max_size);

        let mut b = 1;
        while b < n {
            let mut w = ConstModInt::new(1);

            let mut j = 0;
            let mut k: u32 = 1;
            while j < n {
                for i in 0..b {
                    let s = f[i + j];
                    let t = f[i + j + b];

                    f[i + j] = s + t;
                    f[i + j + b] = (s - t) * w;
                }
                w *= self.inv_base[k.trailing_zeros() as usize];
                j += 2 * b;
                k += 1;
            }
            b <<= 1;
        }

        let t = ConstModInt::new(n as u32).inv();
        for x in f.iter_mut() {
            *x *= t;
        }
    }

    pub fn convolve(
        &self,
        mut f: Vec<ConstModInt<P>>,
        mut g: Vec<ConstModInt<P>>,
    ) -> Vec<ConstModInt<P>> {
        let m = f.len() + g.len() - 1;
        let n = if m.is_power_of_two() {
            m
        } else {
            m.next_power_of_two()
        };

        f.resize(n, ConstModInt::new(0));
        self.run(&mut f);

        g.resize(n, ConstModInt::new(0));
        self.run(&mut g);

        for (f, g) in f.iter_mut().zip(g.into_iter()) {
            *f *= g;
        }
        self.run_inv(&mut f);

        f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::ff::const_modint::*;
    use rand::Rng;

    #[test]
    fn test() {
        const MOD: u32 = 998244353;

        let ntt = NTT::<MOD, 3>::new(1 << 20);
        let ff = ConstModIntBuilder::<MOD>::new();

        let mut rng = rand::thread_rng();

        let n = rng.gen_range(1..100);
        let m = rng.gen_range(1..100);

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
