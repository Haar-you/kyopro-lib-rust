use std::collections::VecDeque;
use std::cmp::Reverse;

/// 配列のすべての長さkの連続部分列について、その最小値を列挙する。
///
/// Time complexity O(n)
pub fn sliding_minimum<T: Ord + Clone>(a: &[T], k: usize) -> Vec<T> {
    let n = a.len();

    let mut dq = VecDeque::new();
    let mut ret = Vec::new();

    for i in 0..k {
        while !dq.is_empty() && a[*dq.back().unwrap()] >= a[i] {
            dq.pop_back();
        }
        dq.push_back(i);
    }

    for i in 0..n - k + 1 {
        while *dq.front().unwrap() < i {
            dq.pop_front();
        }

        ret.push(a[*dq.front().unwrap()].clone());

        while !dq.is_empty() && i + k < n && a[*dq.back().unwrap()] >= a[i + k] {
            dq.pop_back();
        }

        dq.push_back(i + k);
    }

    ret
}

pub fn sliding_maximum<T: Ord + Clone>(a: &[T], k: usize) -> Vec<T> {
    let s = a.iter().map(|x| Reverse(x.clone())).collect::<Vec<_>>();
    sliding_minimum(&s, k).into_iter().map(|x| x.0).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(sliding_minimum(&[1, 7, 7, 4, 8, 1, 6], 3), [1, 4, 4, 1, 1]);
        assert_eq!(sliding_maximum(&[1, 7, 7, 4, 8, 1, 6], 3), [7, 7, 8, 8, 8]);
    }
}
