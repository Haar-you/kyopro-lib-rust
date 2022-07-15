use std::ops::{Add, AddAssign, Div, Mul, Neg, SubAssign};

pub fn berlekamp_massey<T>(s: Vec<T>) -> Vec<T>
where
    T: From<u64>
        + Default
        + Copy
        + PartialEq
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + AddAssign
        + SubAssign,
{
    let len = s.len();
    let mut c_ = vec![T::from(1)];
    let mut b_ = vec![T::from(1)];
    let mut l = 0;
    let mut m = 1;
    let mut b = T::from(1);

    for n in 0..len {
        let d = s[n]
            + (1..c_.len())
                .map(|i| c_[i] * s[n - i])
                .fold(T::from(0), Add::add);

        if d == T::from(0) {
            m += 1;
        } else if 2 * l <= n {
            let temp = c_.clone();
            if c_.len() < b_.len() + m {
                c_.resize(b_.len() + m, T::default());
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
                c_.resize(b_.len() + m, T::default());
            }
            let t = d / b;
            for i in 0..b_.len() {
                c_[i + m] -= t * b_[i];
            }
            m += 1;
        }
    }

    (0..l).map(|i| -c_[i + 1]).collect()
}
