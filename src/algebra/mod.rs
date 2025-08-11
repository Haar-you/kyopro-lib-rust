//! 代数的構造

pub mod traits;

pub mod affine;
pub mod bitand;
pub mod bitor;
pub mod bitxor;
pub mod dihedral;
pub mod dual;
pub mod first_last;
pub mod max;
pub mod max_contiguous;
pub mod max_contiguous_many;
pub mod max_partial_sum;
pub mod min;
pub mod min_count;
pub mod option;
pub mod permutation;
pub mod prod;
pub mod sum;
pub mod transform;
pub mod trivial;
pub mod tuple;

pub mod action;

pub mod add_min_count;
pub mod add_sum;
pub mod affine_sum;
pub mod update_fold;
pub mod update_sum;

pub mod semiring;

#[cfg(test)]
mod tests {
    use super::traits::*;
    use std::fmt::Debug;

    fn associative_law<T, I>(a: I)
    where
        T: BinaryOp + Associative + Copy + PartialEq + Debug,
        I: IntoIterator<Item = T>,
    {
        let a: Vec<_> = a.into_iter().collect();
        for x in &a {
            for y in &a {
                for z in &a {
                    let p = x.clone().op(y.clone().op(z.clone()));
                    let q = (x.clone().op(y.clone())).op(z.clone());
                    assert_eq!(p, q)
                }
            }
        }
    }

    fn inverse_law<T, I>(a: I)
    where
        T: BinaryOp + Inverse + Identity + Copy + PartialEq + Debug,
        I: IntoIterator<Item = T>,
    {
        for x in a {
            assert_eq!(x.op(x.inv()), T::id());
            assert_eq!(x.inv().op(x), T::id());
        }
    }

    fn identity_law<T, I>(a: I)
    where
        T: BinaryOp + Identity + Copy + PartialEq + Debug,
        I: IntoIterator<Item = T>,
    {
        for x in a {
            assert_eq!(x.op(T::id()), x);
            assert_eq!(T::id().op(x), x);
        }
    }

    fn commutative_law<T, I>(a: I)
    where
        T: BinaryOp + Commutative + Copy + PartialEq + Debug,
        I: IntoIterator<Item = T>,
    {
        let a: Vec<_> = a.into_iter().collect();
        for x in &a {
            for y in &a {
                assert_eq!(x.op(*y), y.op(*x));
            }
        }
    }

    #[test]
    fn test_dihedral() {
        use crate::algebra::dihedral::*;

        let k = 20;

        let a = (0..k)
            .map(|i| Dihedral::r(i, k))
            .chain((0..k).map(|i| Dihedral::s(i, k)));

        associative_law(a.clone());
        inverse_law(a.clone());
        identity_law(a);
    }

    #[test]
    fn test_sum_modint() {
        use crate::num::modint::{algebra::*, *};

        let m: u32 = 73;
        let ff = ModIntBuilder::new(m);
        let a = (0..m as u64).map(|x| SumModM::new(ff.from_u64(x)));

        associative_law(a.clone());
        inverse_law(a.clone());
        identity_law(a.clone());
        commutative_law(a);
    }

    #[test]
    fn test_prod_modint() {
        use crate::num::modint::{algebra::*, *};

        let m: u32 = 73;
        let ff = ModIntBuilder::new(m);
        let a = (0..m as u64).map(|x| ProdModM::new(ff.from_u64(x)));

        associative_law(a.clone());
        identity_law(a.clone());
        commutative_law(a);
    }
}
