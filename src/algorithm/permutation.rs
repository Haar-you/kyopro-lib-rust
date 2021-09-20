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

pub struct Permutation<T> {
    data: Vec<T>,
    first: bool,
}

impl<T: Ord + Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.first && !next_permutation(&mut self.data) {
            None
        } else {
            self.first = false;
            Some(self.data.clone())
        }
    }
}

pub fn permutations<T: Ord + Clone>(a: &[T]) -> Permutation<T> {
    Permutation {
        data: a.to_vec(),
        first: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = [1, 2, 3];

        assert_eq!(
            permutations(&a).collect::<Vec<_>>(),
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
