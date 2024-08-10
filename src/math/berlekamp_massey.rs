use crate::math::ff::traits::*;

pub fn berlekamp_massey<Modulo: FF>(s: Vec<u32>, modulo: Modulo) -> Vec<Modulo::Output>
where
    Modulo::Output: FFElem,
{
    let len = s.len();
    let mut c_ = vec![modulo.from_u64(1)];
    let mut b_ = vec![modulo.from_u64(1)];
    let mut l = 0;
    let mut m = 1;
    let mut b = modulo.from_u64(1);

    for n in 0..len {
        let d = modulo.from_u64(s[n] as u64)
            + (1..c_.len())
                .map(|i| c_[i] * modulo.from_u64(s[n - i] as u64))
                .fold(modulo.from_u64(0), std::ops::Add::add);

        if d == modulo.from_u64(0) {
            m += 1;
        } else if 2 * l <= n {
            let temp = c_.clone();
            if c_.len() < b_.len() + m {
                c_.resize(b_.len() + m, modulo.from_u64(0));
            }
            let t = d / b;
            for i in 0..b_.len() {
                c_[i + m] -= t * b_[i];
            }
            l = n + 1 - l;
            b_ = temp;
            b = d;
            m = 1;
        } else {
            if c_.len() < b_.len() + m {
                c_.resize(b_.len() + m, modulo.from_u64(0));
            }
            let t = d / b;
            for i in 0..b_.len() {
                c_[i + m] -= t * b_[i];
            }
            m += 1;
        }
    }

    c_.into_iter().skip(1).take(l).map(|x| -x).collect()
}
