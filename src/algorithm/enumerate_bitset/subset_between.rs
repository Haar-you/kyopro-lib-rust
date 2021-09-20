use std::iter::successors;

pub fn subset_between(a: u32, b: u32) -> impl Iterator<Item = u32> {
    let x = b ^ (a & b);

    successors(if a & !b != 0 { None } else { Some(0) }, move |&t| {
        if t == 0 {
            None
        } else {
            Some((t - 1) % x)
        }
    })
    .map(move |t| t | a)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(x: u32, y: u32) {
        let a = (0..=x)
            .filter(|i| (x & !i) == 0 && (!y & i) == 0)
            .collect::<Vec<_>>();

        let b = subset_between(x, y).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(0b11111111, 0b11111111);
        check(0b00000000, 0b11111111);
        check(0b10101010, 0b11111111);
        check(0b00000001, 0b01010101);
        check(0b00000001, 0b00000010);
    }
}
