pub fn run_length_encoding<T>(a: &[T]) -> Vec<(T, usize)>
where
    T: Eq + Clone,
{
    let mut ret: Vec<(T, usize)> = vec![];

    for x in a {
        match ret.last_mut() {
            Some((y, c)) if y == x => *c += 1,
            _ => ret.push((x.clone(), 1)),
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
            run_length_encoding(&a),
            [(1, 2), (2, 1), (1, 1), (3, 1), (4, 1), (5, 2), (4, 1)]
        );
    }

    #[test]
    fn test_str() {
        let a = "aabbccd";
        assert_eq!(
            run_length_encoding(&a.chars().collect::<Vec<_>>()),
            [('a', 2), ('b', 2), ('c', 2), ('d', 1)]
        );
    }
}
