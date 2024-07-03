//! 並列二分探索
//!
//! # Problems
//!
//! - [CODE THANKS FESTIVAL 2017 H - Union Sets](https://atcoder.jp/contests/code-thanks-festival-2017-open/submissions/55116191)

pub fn parallel_binary_search(
    m: usize,
    q: usize,
    mut init: impl FnMut(),
    mut process: impl FnMut(usize),
    mut checker: impl FnMut(usize) -> bool,
) -> Vec<usize> {
    let mut ok: Vec<isize> = vec![m as isize; q];
    let mut ng: Vec<isize> = vec![-1; q];

    loop {
        let mut check = true;
        let mut mids = vec![vec![]; m];

        for i in 0..q {
            if ok[i] - ng[i] > 1 {
                check = false;
                let mid = (ok[i] + ng[i]) / 2;
                mids[mid as usize].push(i);
            }
        }

        if check {
            break;
        }

        init();

        for i in 0..m {
            process(i);
            for &j in &mids[i] {
                if checker(j) {
                    ok[j] = i as isize;
                } else {
                    ng[j] = i as isize;
                }
            }
        }
    }

    ok.into_iter().map(|x| x as usize).collect()
}
