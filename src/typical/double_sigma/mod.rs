pub mod difference;
pub mod range_xor;

#[cfg(test)]
mod tests {
    use std::ops::AddAssign;

    use rand::Rng;

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

    #[test]
    fn test_difference() {
        let mut rng = rand::thread_rng();
        let n = 300;
        let a = (0..n).map(|_| rng.gen::<i32>() as i64).collect::<Vec<_>>();

        let res = super::difference::sum_of_sum_of_difference(a.clone());
        let ans = solve(a, 0, |a, i, j| (a[i] - a[j]).abs());

        assert_eq!(res, ans);
    }
}
