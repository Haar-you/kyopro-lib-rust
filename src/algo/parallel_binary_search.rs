//! 並列二分探索
//!
//! # Problems
//!
//! - [CODE THANKS FESTIVAL 2017 H - Union Sets](https://atcoder.jp/contests/code-thanks-festival-2017-open/submissions/55116191)

/// 並列二分探索
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

        for (i, (ok, ng)) in ok.iter().zip(&ng).enumerate() {
            if ok - ng > 1 {
                check = false;
                let mid = (ok + ng) / 2;
                mids[mid as usize].push(i);
            }
        }

        if check {
            break;
        }

        init();

        for (i, mid) in mids.iter().enumerate() {
            process(i);
            for &j in mid {
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
