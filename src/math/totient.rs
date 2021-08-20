pub fn totient(mut n: u64) -> u64 {
    let mut ret = n;

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            ret -= ret / i;
            while n % i == 0 {
                n /= i;
            }
        }

        i += 1;
    }

    if n != 1 {
        ret -= ret / n;
    }

    ret
}

pub fn totient_table(n: usize) -> Vec<usize> {
    let mut ret = (0..=n).collect::<Vec<usize>>();

    for i in 2..=n {
        if ret[i] == i {
            for j in (i..=n).step_by(i) {
                ret[j] = ret[j] / i * (i - 1);
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // https://oeis.org/A000010/list
        assert_eq!(
            totient_table(69)[1..],
            [
                1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20,
                12, 18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22,
                46, 16, 42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66,
                32, 44
            ]
        );
    }
}
