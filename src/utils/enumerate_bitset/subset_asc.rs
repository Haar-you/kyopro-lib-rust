use std::iter::successors;

pub fn subset_asc(a: u32) -> impl Iterator<Item = u32> {
    successors(Some(0), move |&t| {
        if t == a {
            None
        } else {
            Some(((t as i32 - a as i32) & (a as i32)) as u32)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(x: u32) {
        let a = (0..=x).filter(|i| (!x & i) == 0).collect::<Vec<_>>();

        let b = subset_asc(x).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(0b11111111);
        check(0b00000000);
        check(0b10101010);
        check(0b00000001);
        check(0b10000000);
        check(0b10000001);
        check(0b11011011);
    }
}
