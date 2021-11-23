use std::iter::successors;

pub fn subset_size_k(width: u32, k: u32) -> impl Iterator<Item = u32> {
    successors(Some((1 << k) - 1), move |&t| {
        let x = ((t as i32) & (-(t as i32))) as u32;
        let y = t + x;
        let t = ((t & !y) / x) >> 1 | y;
        if t >= 1 << width {
            None
        } else {
            Some(t)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(n: u32, k: u32) {
        let a = (0..1 << n)
            .filter(|&i| (i as u32).count_ones() == k)
            .collect::<Vec<_>>();

        let b = subset_size_k(n, k).collect::<Vec<_>>();

        assert_eq!(a, b);
    }

    #[test]
    fn test() {
        check(10, 3);
    }
}
