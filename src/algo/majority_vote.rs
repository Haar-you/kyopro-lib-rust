//! Boyer-Moore majority vote algorithm

/// Boyer-Moore majority vote algorithm
///
/// **Time complexity** $O(n)$
pub fn majority_vote<T: Eq>(a: &[T]) -> Option<(&T, usize)> {
    let mut candidate = None;
    let mut counter = 0;

    for x in a {
        if counter == 0 {
            candidate = Some(x);
            counter = 1;
        } else {
            match candidate {
                Some(y) if x == y => counter += 1,
                _ => counter -= 1,
            }
        }
    }

    if let Some(x) = candidate {
        let count = a.iter().filter(|&y| x == y).count();
        if count <= a.len() / 2 {
            None
        } else {
            Some((x, count))
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::collections::BTreeMap;

    fn check<T: Eq + Ord>(a: &[T]) -> Option<(&T, usize)> {
        let mut map = BTreeMap::<&T, usize>::new();
        let h = a.len() / 2;

        for x in a {
            *map.entry(x).or_insert(0) += 1;
        }

        for (k, v) in map {
            if v > h {
                return Some((k, v));
            }
        }
        None
    }

    #[test]
    fn test() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let n = rng.gen_range(0..=100);
            let a = std::iter::repeat_with(|| rng.gen::<u64>() % 10)
                .take(n)
                .collect::<Vec<_>>();

            assert_eq!(check(&a), majority_vote(&a));
        }
    }
}
