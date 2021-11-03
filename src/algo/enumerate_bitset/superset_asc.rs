use std::iter::successors;

pub fn super_asc(a: u32, n: u32) -> impl Iterator<Item = u32> {
    successors(Some(a), move |&t| {
        let t = (t + 1) | a;
        if t >= 1 << n {
            None
        } else {
            Some(t)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(x: u32, n: u32) {
        let a = (0..1 << n).filter(|i| (x & !i) == 0).collect::<Vec<_>>();

        let b = super_asc(x, n).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(0b11111111, 8);
        check(0b00000000, 8);
        check(0b10101010, 8);
        check(0b00000001, 8);
        check(0b10000000, 8);
        check(0b10000001, 8);
        check(0b11011011, 8);
    }
}
