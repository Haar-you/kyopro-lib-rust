use std::cmp::max;

pub fn lcs<T: Copy + PartialEq>(a: &[T], b: &[T]) -> Vec<T> {
    let n = a.len();
    let m = b.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = max(dp[i][j], dp[i - 1][j - 1] + 1);
            } else {
                dp[i][j] = max(dp[i][j], dp[i - 1][j]);
                dp[i][j] = max(dp[i][j], dp[i][j - 1]);
            }
        }
    }

    let mut ret = Vec::with_capacity(dp[n][m]);
    lcs_restore(&dp, a, b, n, m, &mut ret);

    ret
}

fn lcs_restore<T: Copy + PartialEq>(
    dp: &[Vec<usize>],
    a: &[T],
    b: &[T],
    i: usize,
    j: usize,
    ret: &mut Vec<T>,
) {
    if i == 0 || j == 0 {
        return;
    }
    if a[i - 1] == b[j - 1] {
        lcs_restore(dp, a, b, i - 1, j - 1, ret);
        ret.push(a[i - 1]);
    } else if dp[i - 1][j] >= dp[i][j - 1] {
        lcs_restore(dp, a, b, i - 1, j, ret);
    } else {
        lcs_restore(dp, a, b, i, j - 1, ret);
    }
}
