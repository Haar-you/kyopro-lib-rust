//! ナップサック問題
//!
//! | function | time complexity | space complexity |
//! | ---- | ---- | ---- |
//! | knapsack_small_weight | $O(n \cdot cap)$ | $O(cap)$ |
//! | knapsack_small_value | $O(n \sum vs)$ | $O(\sum vs)$ |
//! | knapsack_small_quantity | $O(n 2^{n / 2})$ | $O(2^{n / 2})$ |
//! | knapsack_limited | $O(n \cdot cap \log \max ms)$ | $O(cap)$ |
//! | knapsack_unlimited | $O(n \cdot cap)$ | $O(cap)$ |

pub mod limited;
pub mod small_quantity;
pub mod small_value;
pub mod small_weight;
pub mod unlimited;

#[cfg(test)]
mod tests {
    use super::{limited::*, small_quantity::*, small_value::*, small_weight::*, unlimited::*};

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_1_B
        assert_eq!(knapsack_small_weight(5, &[2, 2, 1, 3], &[4, 5, 2, 8]), 13);
        assert_eq!(knapsack_small_weight(20, &[9, 10], &[5, 4]), 9);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_1_F
        assert_eq!(knapsack_small_value(5, &[2, 2, 1, 3], &[4, 5, 2, 8]), 13);
        assert_eq!(knapsack_small_value(20, &[9, 10], &[5, 4]), 9);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_G
        assert_eq!(
            knapsack_limited(8, &[3, 1, 2, 2], &[4, 2, 1, 3], &[2, 1, 4, 2]),
            12
        );
        assert_eq!(knapsack_limited(100, &[1, 1], &[1, 2], &[100, 50]), 150);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_C
        assert_eq!(knapsack_unlimited(8, &[2, 2, 1, 3], &[4, 5, 2, 8]), 21);
        assert_eq!(knapsack_unlimited(20, &[9, 10], &[5, 4]), 10);
        assert_eq!(knapsack_unlimited(9, &[1, 1, 2], &[2, 3, 5]), 27);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_H
        assert_eq!(knapsack_small_quantity(5, &[2, 2, 1, 3], &[4, 5, 2, 8]), 13);
        assert_eq!(knapsack_small_quantity(20, &[9, 10], &[5, 4]), 9);
    }
}
