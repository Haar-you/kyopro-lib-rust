pub mod bell_number;
pub mod bell_number_table;
pub mod bernoulli_number;
pub mod montmort;
pub mod partition_number;
pub mod stirling_first;
pub mod stirling_first_fixed_k;
pub mod stirling_first_table;
pub mod stirling_second;
pub mod stirling_second_fixed_k;
pub mod stirling_second_table;

#[cfg(test)]
mod tests {

    use crate::math::combinatorics::stirling_first::stirling_first;
    use crate::math::combinatorics::stirling_first_fixed_k::stirling_first_fixed_k;
    use crate::math::combinatorics::stirling_first_table::stirling_first_table;
    use crate::math::combinatorics::stirling_second::stirling_second;
    use crate::math::combinatorics::stirling_second_fixed_k::stirling_second_fixed_k;
    use crate::math::combinatorics::stirling_second_table::stirling_second_table;
    use crate::math::prime_mod::Prime;
    use crate::num::const_modint::ConstModIntBuilder;

    type P = Prime<998244353>;

    #[test]
    fn test_stirling_first() {
        let n = 100;

        let table = stirling_first_table(n, ConstModIntBuilder::<P>::new());

        for (i, ans) in table.iter().enumerate() {
            let row = stirling_first(i);

            assert_eq!(ans[0..=i], row);
        }

        for k in 0..=n {
            let col = stirling_first_fixed_k(n, k);
            let ans = table.iter().map(|a| a[k]).collect::<Vec<_>>();

            assert_eq!(ans, col);
        }
    }

    #[test]
    fn test_stirling_second() {
        let n = 100;

        let table = stirling_second_table(n, ConstModIntBuilder::<P>::new());

        for (i, ans) in table.iter().enumerate() {
            let row = stirling_second(i);

            assert_eq!(ans[0..=i], row);
        }

        for k in 0..=n {
            let col = stirling_second_fixed_k(n, k);
            let ans = table.iter().map(|a| a[k]).collect::<Vec<_>>();

            assert_eq!(ans, col);
        }
    }
}
