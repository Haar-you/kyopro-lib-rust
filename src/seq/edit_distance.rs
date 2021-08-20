use crate::chmin;

pub fn edit_distance<T: PartialEq>(a: &[T], b: &[T]) -> usize {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..=n {
        dp[i][0] = i;
    }
    for i in 0..=m {
        dp[0][i] = i;
    }

    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = std::cmp::min(dp[i][j + 1] + 1, dp[i + 1][j] + 1);

            if a[i] == b[j] {
                chmin!(dp[i + 1][j + 1], dp[i][j]);
            } else {
                chmin!(dp[i + 1][j + 1], dp[i][j] + 1);
            }
        }
    }

    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(edit_distance("acac".as_bytes(), "acm".as_bytes()), 2);
        assert_eq!(edit_distance("icpc".as_bytes(), "icpc".as_bytes()), 0);
    }
}
