use std::ops::Sub;

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

pub fn fast_mobius_subset<T>(f: &mut [T])
where
    T: Copy + Sub<Output = T>,
{
    let n = f.len();
    let t = n.trailing_zeros();
    assert!(n.is_power_of_two());

    for i in 0..t {
        let i = 1 << i;
        for j in 0..n {
            if j & i != 0 {
                f[j] = f[j] - f[j ^ i];
            }
        }
    }
}
