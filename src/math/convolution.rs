use std::ops::{Add, Mul, Sub};

pub fn fast_mobius_superset<T>(f: &mut [T])
where
    T: Copy + Sub<Output = T>,
{
    let n = f.len();
    let t = n.trailing_zeros();
    assert!(n.is_power_of_two());

    for i in 0..t {
        let i = 1 << i;
        for j in 0..n {
            if j & i == 0 {
                f[j] = f[j] - f[j ^ i];
            }
        }
    }
}

pub fn fast_zeta_superset<T>(f: &mut [T])
where
    T: Copy + Add<Output = T>,
{
    let n = f.len();
    let t = n.trailing_zeros();
    assert!(n.is_power_of_two());

    for i in 0..t {
        let i = 1 << i;
        for j in 0..n {
            if j & i == 0 {
                f[j] = f[j] + f[j ^ i];
            }
        }
    }
}

pub fn convolution_and<T>(mut f: Vec<T>, mut g: Vec<T>) -> Vec<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    assert!(f.len() == g.len());
    fast_zeta_superset(&mut f);
    fast_zeta_superset(&mut g);
    for i in 0..f.len() {
        f[i] = f[i] * g[i];
    }
    fast_mobius_superset(&mut f);
    f
}
