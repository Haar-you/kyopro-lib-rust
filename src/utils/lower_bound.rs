pub fn lower_bound<T: Clone + Ord + PartialEq>(a: &Vec<T>, value: T) -> usize {
    let n = a.len();
    let mut lb = 0;
    let mut len = n;

    while len > 0 {
        let half = len / 2;
        let mid = lb + half;

        if a[mid] < value {
            len -= half + 1;
            lb = mid + 1;
        }
        else {
            len = half;
        }
    }

    lb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![1, 2, 3, 5, 6];
        assert_eq!(lower_bound(&a, 2), 1);
        assert_eq!(lower_bound(&a, 0), 0);
        assert_eq!(lower_bound(&a, 4), 3);
        assert_eq!(lower_bound(&a, 7), 5);
    }
}
