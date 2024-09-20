pub mod difference;
pub mod max;
pub mod prod;
pub mod range_prod;
pub mod range_sum;
pub mod range_xor;
pub mod sum;
pub mod xor;

#[cfg(test)]
mod tests {
    use std::ops::AddAssign;

    use rand::Rng;

    use crate::num::const_modint::*;

    const M998244353: u32 = 998244353;

    fn solve<T, U, F>(a: Vec<T>, init: U, mut f: F) -> U
    where
        U: AddAssign,
        F: FnMut(&[T], usize, usize) -> U,
    {
        let mut ans = init;
        let n = a.len();

        for i in 0..n {
            for j in i + 1..n {
                ans += f(&a, i, j);
            }
        }

        ans
    }

    fn solve_range<T, U, F>(a: Vec<T>, init: U, mut f: F) -> U
    where
        U: AddAssign,
        F: FnMut(&[T], usize, usize) -> U,
    {
        let mut ans = init;
        let n = a.len();

        for i in 0..n {
            for j in i + 1..=n {
                ans += f(&a, i, j);
            }
        }

        ans
    }

    #[test]
    fn test_difference() {
        let mut rng = rand::thread_rng();
        let n = 300;
        let a = (0..n).map(|_| rng.gen::<i32>() as i64).collect::<Vec<_>>();

        let res = super::difference::sum_of_sum_of_difference(a.clone());
        let ans = solve(a, 0, |a, i, j| (a[i] - a[j]).abs());

        assert_eq!(res, ans);
    }

    #[test]
    fn test_xor() {
        let mut rng = rand::thread_rng();
        let n = 300;
        let a = (0..n).map(|_| rng.gen::<u32>() as u64).collect::<Vec<_>>();

        let res = super::xor::sum_of_sum_of_xor(a.clone()) as u64;
        let ans = solve(a, 0, |a, i, j| a[i] ^ a[j]);

        assert_eq!(res, ans);
    }

    #[test]
    fn test_range_xor() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let a = (0..n)
            .map(|_| rng.gen::<u64>() % 2_u64.pow(32))
            .collect::<Vec<_>>();

        let res = super::range_xor::sum_of_sum_of_range_xor(a.clone()) as u64;
        let ans = solve_range(a, 0, |a, i, j| a[i..j].iter().fold(0, |x, y| x ^ y));

        assert_eq!(res, ans);
    }

    #[test]
    fn test_sum() {
        let mut rng = rand::thread_rng();
        let n = 300;
        let a = (0..n).map(|_| rng.gen::<i32>() as i64).collect::<Vec<_>>();

        let res = super::sum::sum_of_sum_of_sum(a.clone());
        let ans = solve(a, 0, |a, i, j| a[i] + a[j]);

        assert_eq!(res, ans);
    }

    #[test]
    fn test_prod() {
        let mut rng = rand::thread_rng();
        let n = 300;
        let modulo = ConstModIntBuilder::<M998244353>;
        let a = (0..n)
            .map(|_| modulo.from_i64(rng.gen::<i64>()))
            .collect::<Vec<_>>();

        let res = super::prod::sum_of_sum_of_prod(a.clone());
        let ans = solve(a, modulo.from_u64(0), |a, i, j| a[i] * a[j]);

        assert_eq!(res, ans);
    }

    #[test]
    fn test_range_sum() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let a = (0..n).map(|_| rng.gen::<i32>() as i64).collect::<Vec<_>>();

        let res = super::range_sum::sum_of_sum_of_range_sum(a.clone());
        let ans = solve_range(a, 0, |a, i, j| a[i..j].iter().sum());

        assert_eq!(res, ans);
    }

    #[test]
    fn test_max() {
        let mut rng = rand::thread_rng();
        let n = 300;
        let a = (0..n).map(|_| rng.gen::<i32>() as i64).collect::<Vec<_>>();

        let res = super::max::sum_of_sum_of_max(a.clone());
        let ans = solve(a, 0, |a, i, j| a[i].max(a[j]));

        assert_eq!(res, ans);
    }

    #[test]
    fn test_range_prod() {
        let mut rng = rand::thread_rng();
        let n = 100;
        let modulo = ConstModIntBuilder::<M998244353>;
        let a = (0..n)
            .map(|_| modulo.from_i64(rng.gen::<i64>()))
            .collect::<Vec<_>>();

        let res = super::range_prod::sum_of_sum_of_range_prod(a.clone());
        let ans = solve_range(a, modulo.from_u64(0), |a, i, j| {
            a[i..j].iter().fold(modulo.from_u64(1), |x, &y| x * y)
        });

        assert_eq!(res, ans);
    }
}
