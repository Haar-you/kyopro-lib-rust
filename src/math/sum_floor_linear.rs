//! $\sum_{i=0}^{n-1} \lfloor \frac{ai+b}{m} \rfloor$
pub fn sum_floor_linear(n: u64, m: u64, mut a: u64, mut b: u64) -> u64 {
    let mut ret = 0;

    if b >= m {
        ret += n * (b / m);
        b %= m;
    }

    if a >= m {
        ret += n * (n - 1) * (a / m) / 2;
        a %= m;
    }

    let y_max = (a * n + b) / m;
    if y_max == 0 {
        return ret;
    }

    let x_max = y_max * m - b;

    ret += (n - (x_max + a - 1) / a) * y_max;
    ret += sum_floor_linear(y_max, a, m, (a - x_max % a) % a);

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        // https://judge.yosupo.jp/problem/sum_of_floor_of_linear
        assert_eq!(sum_floor_linear(4, 10, 6, 3), 3);
        assert_eq!(sum_floor_linear(6, 5, 4, 3), 13);
        assert_eq!(sum_floor_linear(1, 1, 0, 0), 0);
        assert_eq!(sum_floor_linear(31415, 92653, 58979, 32384), 314095480);
        assert_eq!(
            sum_floor_linear(1000000000, 1000000000, 999999999, 999999999),
            499999999500000000
        );
    }
}
