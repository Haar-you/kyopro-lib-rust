use std::iter::{from_fn, once};

pub fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();

    if n <= 1 {
        false
    } else {
        let i = (0..n - 1).rev().find(|&i| a[i] < a[i + 1]);

        match i {
            None => false,
            Some(i) => {
                let j = (i + 1..n).rev().find(|&j| a[j] > a[i]).unwrap();

                a.swap(i, j);
                a[i + 1..].reverse();

                true
            }
        }
    }
}

pub fn prev_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();

    if n <= 1 {
        false
    } else {
        let i = (0..n - 1).rev().find(|&i| a[i] > a[i + 1]);

        match i {
            None => false,
            Some(i) => {
                let j = (i + 1..n).rev().find(|&j| a[j] < a[i]).unwrap();

                a.swap(i, j);
                a[i + 1..].reverse();

                true
            }
        }
    }
}

pub fn permutations<T: Ord + Clone>(mut a: Vec<T>) -> impl Iterator<Item = Vec<T>> {
    once(a.clone()).chain(from_fn(move || {
        if next_permutation(&mut a) {
            Some(a.clone())
        } else {
            None
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [1, 2, 3, 4, 5];

        for a in permutations(a.to_vec()) {
            let mut b = a.clone();
            if next_permutation(&mut b) {
                prev_permutation(&mut b);
                assert_eq!(a, b);
            }
        }

        let a = [1, 2, 3];

        assert_eq!(
            permutations(a.to_vec()).collect::<Vec<_>>(),
            [
                [1, 2, 3],
                [1, 3, 2],
                [2, 1, 3],
                [2, 3, 1],
                [3, 1, 2],
                [3, 2, 1]
            ]
        );
    }
}
