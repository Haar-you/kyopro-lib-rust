//! 最大連続部分和
use std::ops::Add;
use std::ops::Range;

/// 空でない連続する部分列の和で最大のものを返す。
///
/// **Time Complexity O(|a|)**
pub fn max_partial_sum<T>(a: &[T]) -> Option<(T, Range<usize>)>
where
    T: Copy + Ord + Add<Output = T>,
{
    let mut t = a.first().copied()?;
    let mut ret = t;

    let mut t_l = 0;
    let mut l = 0;
    let mut r = 0;

    for (i, &x) in a.iter().enumerate().skip(1) {
        if t + x < x {
            t = x;
            t_l = i;
        } else {
            t = t + x;
        }

        if ret < t {
            ret = t;
            l = t_l;
            r = i + 1;
        }
    }

    Some((ret, l..r))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_zero_array() {
        assert_eq!(max_partial_sum::<i64>(&vec![]), None);
    }

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        let n = 100;

        let a = (0..n)
            .map(|_| rng.gen_range(-1000..=1000))
            .collect::<Vec<_>>();

        let mut ans = std::i64::MIN;

        for i in 0..n {
            for j in i + 1..=n {
                ans = ans.max(a[i..j].iter().sum());
            }
        }

        let (res, range) = max_partial_sum(&a).unwrap();

        assert_eq!(res, ans);
        assert_eq!(a[range].iter().sum::<i64>(), ans);
    }

    #[test]
    fn test_all_negative() {
        let mut rng = rand::thread_rng();

        let n = 100;

        let a = (0..n).map(|_| rng.gen_range(-1000..0)).collect::<Vec<_>>();

        let mut ans = std::i64::MIN;

        for i in 0..n {
            for j in i + 1..=n {
                ans = ans.max(a[i..j].iter().sum());
            }
        }

        let (res, range) = max_partial_sum(&a).unwrap();

        assert_eq!(res, ans);
        assert_eq!(a[range].iter().sum::<i64>(), ans);
    }
}
