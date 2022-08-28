use crate::math::totient::totient_table;
use std::collections::HashMap;

pub fn totient_sum(n: u64, m: u64) -> u64 {
    let k = (n as f64).powf(0.66) as u64;

    let mut memo1 = vec![0; k as usize + 1];
    let table = totient_table(k as usize);
    let mut sum = 0;
    for i in 1..=k as usize {
        sum = (sum + table[i]) % m;
        memo1[i] = sum;
    }

    let mut memo2 = HashMap::new();

    rec(n, m, &memo1, &mut memo2)
}

fn rec(x: u64, m: u64, memo1: &[u64], memo2: &mut HashMap<u64, u64>) -> u64 {
    if let Some(y) = memo1.get(x as usize) {
        return *y;
    }
    if let Some(y) = memo2.get(&x) {
        return *y;
    }

    let mut ret = if x % 2 == 0 {
        (x / 2) % m * (x + 1) % m
    } else {
        x % m * ((x + 1) / 2) % m
    };

    let s = (x as f64).sqrt() as u64;

    for i in 2..=s {
        if x / i > s {
            ret = (m + ret - rec(x / i, m, memo1, memo2)) % m;
        }
    }

    for i in 1..=s {
        let t = (x / i - x / (i + 1)) % m;
        ret = (m + ret - rec(i, m, memo1, memo2) * t % m) % m;
    }

    memo2.insert(x, ret);
    ret
}
