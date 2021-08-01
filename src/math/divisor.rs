pub fn count_divisors(n: u64) -> u64 {
    let mut ret = 0;

    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            ret += 2;
            if i * i == n {
                ret -= 1;
            }
        }

        i += 1;
    }

    ret
}

pub fn enumerate_divisors(n: u64) -> Vec<u64> {
    let mut ret = vec![];
    let mut temp = vec![];

    let mut i = 1;
    while i * i < n {
        if n % i == 0 {
            temp.push(n / i);
            ret.push(i);
        }

        i += 1;
    }

    if i * i == n {
        ret.push(i);
    }

    temp.reverse();
    ret.extend(&temp);

    ret
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 24;
        assert_eq!(enumerate_divisors(n), (1 ..= n).filter(|i| n % i == 0).collect::<Vec<_>>());
    }
}
