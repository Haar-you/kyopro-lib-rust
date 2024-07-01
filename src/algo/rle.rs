//! Run Length Encoding

pub fn rle<T: PartialEq>(a: &[T]) -> Vec<(&T, usize)> {
    let mut ret = vec![];

    for x in a {
        match ret.last_mut() {
            Some((y, c)) if x == *y => *c += 1,
            _ => ret.push((x, 1)),
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let a = vec![1, 1, 2, 1, 3, 4, 5, 5, 4];
        assert_eq!(
            rle(&a),
            [
                (&1, 2),
                (&2, 1),
                (&1, 1),
                (&3, 1),
                (&4, 1),
                (&5, 2),
                (&4, 1)
            ]
        );
    }

    #[test]
    fn test_str() {
        let a = "aabbccd";
        assert_eq!(
            rle(&a.chars().collect::<Vec<_>>()),
            [(&'a', 2), (&'b', 2), (&'c', 2), (&'d', 1)]
        );
    }
}
