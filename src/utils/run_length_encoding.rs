
fn run_length_encoding<T>(a: &[T]) -> Vec<(T, usize)>
where
    T: Eq + Clone
{
    let mut ret: Vec<(T, usize)> = vec![];

    for x in a {
        if !ret.is_empty() && ret.last().unwrap().0 == x.clone() {
            ret.last_mut().unwrap().1 += 1;
        }
        else {
            ret.push((x.clone(), 1));
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![1, 1, 2, 1, 3, 4, 5, 5, 4];
        assert_eq!(run_length_encoding(&a), [(1, 2), (2, 1), (1, 1), (3, 1), (4, 1), (5, 2), (4, 1)]);

        let a = "aabbccd";
        assert_eq!(run_length_encoding(&a.chars().collect::<Vec<_>>()), [('a', 2), ('b', 2), ('c', 2), ('d', 1)]);
    }
}
