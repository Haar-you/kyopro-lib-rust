use std::iter::successors;

pub fn superset_desc(a: u32, n: u32) -> impl Iterator<Item = u32> {
    let x = (1 << n) - 1;
    let y = x ^ (a & x);

    successors(Some(y), move |&t| t.checked_sub(1).map(|x| x & y)).map(move |t| t | a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(0b11111111, 8)]
    #[test_case(0b00000000, 8)]
    #[test_case(0b10101010, 8)]
    #[test_case(0b00000001, 8)]
    #[test_case(0b10000000, 8)]
    #[test_case(0b10000001, 8)]
    #[test_case(0b11011011, 8)]
    fn check(x: u32, n: u32) {
        let a = (0..1 << n)
            .rev()
            .filter(|i| (x & !i) == 0)
            .collect::<Vec<_>>();

        let b = superset_desc(x, n).collect::<Vec<_>>();

        assert_eq!(a, b);
    }
}
