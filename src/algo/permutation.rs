//! 順列の列挙
use std::iter::{from_fn, once};

macro_rules! impl_permutation {
    ( $cmp1:tt, $cmp2:tt, $a: expr ) => {{
        let n = $a.len();

        if n <= 1 {
            false
        } else {
            let i = (0..n - 1).rev().find(|&i| $a[i] $cmp1 $a[i + 1]);

            match i {
                None => false,
                Some(i) => {
                    let j = (i + 1..n).rev().find(|&j| $a[j] $cmp2 $a[i]).unwrap();

                    $a.swap(i, j);
                    $a[i + 1..].reverse();

                    true
                }
            }
        }
    }}
}

/// `a`を辞書式順序で次の順列にする。
pub fn next_permutation<T: Ord + Copy>(a: &mut [T]) -> bool {
    impl_permutation!(<, >, a)
}

/// `a`を辞書式順序で前の順列にする。
pub fn prev_permutation<T: Ord + Copy>(a: &mut [T]) -> bool {
    impl_permutation!(>, <, a)
}

/// 辞書式順序で`a`以降の順列を列挙するイテレータを返す。
pub fn permutations<T: Ord + Copy>(mut a: Vec<T>) -> impl Iterator<Item = Vec<T>> {
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
