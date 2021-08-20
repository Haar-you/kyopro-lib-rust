pub fn ext_gcd(a: u64, b: u64) -> (i64, i64, i64) {
    if b == 0 {
        return (a as i64, 1, 0);
    }
    let (d, q, p) = ext_gcd(b, (a + b) % b);
    (d, p, q - (a / b) as i64 * p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test() {
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            let n = rng.gen::<u64>() % 1000;
            let m = rng.gen::<u64>() % 1000;
            let (g, p, q) = ext_gcd(n, m);

            assert_eq!(n as i64 * p + m as i64 * q, g);
        }
    }
}
