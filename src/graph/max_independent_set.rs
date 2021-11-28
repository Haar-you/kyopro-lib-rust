//! 最大独立集合

#[allow(clippy::cognitive_complexity)]
pub fn max_independent_set(g: Vec<Vec<bool>>) -> Vec<usize> {
    let n = g.len();

    let h1 = n / 2;
    let h2 = n - h1;

    let mut dp1 = vec![true; 1 << h1];
    for i in 0..h1 {
        for j in 0..h1 {
            if g[i][j] {
                dp1[(1 << i) | (1 << j)] = false;
            }
        }
    }

    for s in 0..1 << h1 {
        if !dp1[s] {
            for j in 0..h1 {
                dp1[s | (1 << j)] = false;
            }
        }
    }

    let mut dp2 = vec![true; 1 << h2];
    for i in h1..n {
        for j in h1..n {
            if g[i][j] {
                dp2[(1 << (i - h1)) | (1 << (j - h1))] = false;
            }
        }
    }

    for s in 0..1 << h2 {
        if !dp2[s] {
            for j in 0..h2 {
                dp2[s | (1 << j)] = false;
            }
        }
    }

    let mut dp3 = vec![0; 1 << h1];
    dp3[0] = (1 << h2) - 1;

    for i in 0..h1 {
        let mut t = 0;
        for j in h1..n {
            if g[i][j] {
                t |= 1 << (j - h1);
            }
        }
        dp3[1 << i] = t ^ ((1 << h2) - 1);
    }

    for s in 0..1 << h1 {
        for j in 0..h1 {
            if s & (1 << j) == 0 {
                dp3[s | (1 << j)] = dp3[s] & dp3[1 << j];
            }
        }
    }

    let mut dp4 = vec![0; 1 << h2];
    for i in 0..1 << h2 {
        if dp2[i] {
            dp4[i] = i;
        }
    }

    for s in 0..1 << h2 {
        for j in 0..h2 {
            if s & (1 << j) == 0 && dp4[s | (1 << j)].count_ones() <= dp4[s].count_ones() {
                dp4[s | (1 << j)] = dp4[s];
            }
        }
    }

    let mut ans = 0;
    let mut size = 0;

    for s in 0..1 << h1 {
        if dp1[s] {
            let t = s | (dp4[dp3[s]] << h1);

            if t.count_ones() > size {
                size = t.count_ones();
                ans = t;
            }
        }
    }

    (0..64).filter(|i| (ans >> i) & 1 == 1).collect()
}
