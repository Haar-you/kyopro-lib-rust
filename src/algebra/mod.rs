//! 代数的構造

pub mod traits;

pub mod affine;
pub mod bit;
pub mod dihedral;
pub mod dual;
pub mod first_last;
pub mod max_contiguous;
pub mod max_contiguous_true;
pub mod max_partial_sum;
pub mod min_count;
pub mod min_max;
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
pub mod chmax_max;
pub mod chmin_min;
pub mod update_fold;
pub mod update_sum;

pub mod semiring;

#[cfg(test)]
mod tests {
    use crate::algebra::dihedral;

    use super::traits::*;
    use std::fmt::Debug;

    fn associative_law<T, I>(m: &T, a: I)
    where
        T: BinaryOp<Element: Copy + PartialEq + Debug> + Associative,
        I: IntoIterator<Item = T::Element>,
    {
        let a: Vec<_> = a.into_iter().collect();
        for &x in &a {
            for &y in &a {
                for &z in &a {
                    let p = m.op(x, m.op(y, z));
                    let q = m.op(m.op(x, y), z);
                    assert_eq!(p, q)
                }
            }
        }
    }

    fn inverse_law<T, I>(m: &T, a: I)
    where
        T: BinaryOp<Element: Copy + PartialEq + Debug> + Inverse + Identity,
        I: IntoIterator<Item = T::Element>,
    {
        for x in a {
            assert!(m.is_id(&m.op(x, m.inv(x))));
            assert!(m.is_id(&m.op(m.inv(x), x)));
        }
    }

    fn identity_law<T, I>(m: &T, a: I)
    where
        T: BinaryOp<Element: Copy + PartialEq + Debug> + Identity,
        I: IntoIterator<Item = T::Element>,
    {
        for x in a {
            assert_eq!(m.op(x, m.id()), x);
            assert_eq!(m.op(m.id(), x), x);
        }
    }

    fn commutative_law<T, I>(m: &T, a: I)
    where
        T: BinaryOp<Element: Copy + PartialEq + Debug> + Commutative,
        I: IntoIterator<Item = T::Element>,
    {
        let a: Vec<_> = a.into_iter().collect();
        for x in &a {
            for y in &a {
                assert_eq!(m.op(*x, *y), m.op(*y, *x));
            }
        }
    }

    #[test]
    fn test_dihedral() {
        use crate::algebra::dihedral::*;

        let k = 20;
        let m = dihedral::Composition::new(k);

        let a = (0..k)
            .map(|i| Dihedral::r(i, k))
            .chain((0..k).map(|i| Dihedral::s(i, k)));

        associative_law(&m, a.clone());
        inverse_law(&m, a.clone());
        identity_law(&m, a);
    }

    #[test]
    fn test_sum_modint() {
        use crate::num::{algebra::*, modint::*};

        let n: u32 = 73;
        let m = SumMod::new();

        let ff = ModIntBuilder::new(n);
        let a = (0..n as u64).map(|x| ff.from_u64(x));

        associative_law(&m, a.clone());
        inverse_law(&m, a.clone());
        identity_law(&m, a.clone());
        commutative_law(&m, a);
    }

    #[test]
    fn test_prod_modint() {
        use crate::num::{algebra::*, modint::*};

        let n: u32 = 73;
        let m = ProdMod::new();

        let ff = ModIntBuilder::new(n);
        let a = (0..n as u64).map(|x| ff.from_u64(x));

        associative_law(&m, a.clone());
        identity_law(&m, a.clone());
        commutative_law(&m, a);
    }
}
