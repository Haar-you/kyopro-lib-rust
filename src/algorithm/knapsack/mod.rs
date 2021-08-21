pub mod small_weight;
pub mod small_value;
pub mod limited;
pub mod unlimited;
pub mod small_quantity;

#[cfg(test)]
mod tests {
    use super::{
        small_weight::*,
        small_value::*,
        limited::*,
        unlimited::*,
        small_quantity::*,
    };

    #[test]
    fn test() {
        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_1_B
        assert_eq!(
            knapsack_small_weight(4, 5, &[2, 2, 1, 3], &[4, 5, 2, 8]),
            13
        );
        assert_eq!(knapsack_small_weight(2, 20, &[9, 10], &[5, 4]), 9);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_1_F
        assert_eq!(knapsack_small_value(4, 5, &[2, 2, 1, 3], &[4, 5, 2, 8]), 13);
        assert_eq!(knapsack_small_value(2, 20, &[9, 10], &[5, 4]), 9);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_G
        assert_eq!(
            knapsack_limited(4, 8, &[3, 1, 2, 2], &[4, 2, 1, 3], &[2, 1, 4, 2]),
            12
        );
        assert_eq!(knapsack_limited(2, 100, &[1, 1], &[1, 2], &[100, 50]), 150);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_C
        assert_eq!(knapsack_unlimited(4, 8, &[2, 2, 1, 3], &[4, 5, 2, 8]), 21);
        assert_eq!(knapsack_unlimited(2, 20, &[9, 10], &[5, 4]), 10);
        assert_eq!(knapsack_unlimited(3, 9, &[1, 1, 2], &[2, 3, 5]), 27);

        // https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/1/DPL_1_H
        assert_eq!(
            knapsack_small_quantity(4, 5, &[2, 2, 1, 3], &[4, 5, 2, 8]),
            13
        );
        assert_eq!(knapsack_small_quantity(2, 20, &[9, 10], &[5, 4]), 9);
    }
}
