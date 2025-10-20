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
    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = rng.gen_range(10..100);
        let m = rng.gen_range(10..100);

        let mut a = vec![
            std::iter::repeat_with(|| rng.gen::<u64>())
                .take(m)
                .collect::<Vec<_>>();
            n
        ];

        for _ in 0..1000 {
            let i1 = rng.gen_range(0..n);
            let i2 = rng.gen_range(0..n);
            let j1 = rng.gen_range(0..m);
            let j2 = rng.gen_range(0..m);

            swap_vv(&mut a, i1, j1, i2, j2);
        }
    }
}
