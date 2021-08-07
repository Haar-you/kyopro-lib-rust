pub fn lower_bound<T: Ord>(a: &[T], value: &T) -> usize {
    let n = a.len();
    let mut lb = 0;
    let mut len = n;

    while len > 0 {
        let half = len / 2;
        let mid = lb + half;

        if &a[mid] < value {
            len -= half + 1;
            lb = mid + 1;
        }
        else {
            len = half;
        }
    }

    lb
}

pub fn upper_bound<T: Ord>(a: &[T], value: &T) -> usize {
    let n = a.len();
    let mut ub = 0;
    let mut len = n;

    while len > 0 {
        let half = len / 2;
        let mid = ub + half;

        if &a[mid] <= value {
            len -= half + 1;
            ub = mid + 1;
        }
        else {
            len = half;
        }
    }

    ub
}

pub fn equal_range<T: Ord>(a: &[T], value: &T) -> (usize, usize) {
    (lower_bound(a, value), upper_bound(a, value))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let a = vec![1, 1, 2, 3, 5, 6];
        assert_eq!(lower_bound(&a, &1), 0);
        assert_eq!(lower_bound(&a, &2), 2);
        assert_eq!(lower_bound(&a, &0), 0);
        assert_eq!(lower_bound(&a, &4), 4);
        assert_eq!(lower_bound(&a, &7), 6);
    }

    #[test]
    fn test_upper_bound() {
        let a = vec![1, 1, 2, 3, 5, 6];
        assert_eq!(upper_bound(&a, &1), 2);
        assert_eq!(upper_bound(&a, &2), 3);
        assert_eq!(upper_bound(&a, &0), 0);
        assert_eq!(upper_bound(&a, &4), 4);
        assert_eq!(upper_bound(&a, &7), 6);
    }

    #[test]
    fn test_equal_range() {
        let a = vec![1, 1, 3, 4, 5, 5, 5, 8];
        assert_eq!(equal_range(&a, &1), (0, 2));
        assert_eq!(equal_range(&a, &5), (4, 7));
        assert_eq!(equal_range(&a, &4), (3, 4));
        assert_eq!(equal_range(&a, &6), (7, 7));
    }
}
