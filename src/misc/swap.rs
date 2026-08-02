//! 二次元`Vec`の要素swap

/// `a[i1][j1]`と`a[i2][j2]`を交換する。
pub fn swap_vv<T>(a: &mut [Vec<T>], i1: usize, j1: usize, i2: usize, j2: usize) {
    let p1: *mut T = &mut a[i1][j1];
    let p2: *mut T = &mut a[i2][j2];

    unsafe {
        p1.swap(p2);
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    use super::*;

    #[test]
    fn test() {
        let mut rng = rand::rng();

        let n = rng.random_range(10..100);
        let m = rng.random_range(10..100);

        let mut a = vec![
            std::iter::repeat_with(|| rng.random::<u64>())
                .take(m)
                .collect::<Vec<_>>();
            n
        ];

        for _ in 0..1000 {
            let i1 = rng.random_range(0..n);
            let i2 = rng.random_range(0..n);
            let j1 = rng.random_range(0..m);
            let j2 = rng.random_range(0..m);

            swap_vv(&mut a, i1, j1, i2, j2);
        }
    }
}
