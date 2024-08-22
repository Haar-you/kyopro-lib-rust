use crate::num::one_zero::*;
pub use crate::num::total_f64::*;

impl Zero for Totalf64 {
    type Output = Self;
    fn zero() -> Self::Output {
        Totalf64(0.0)
    }
}

impl One for Totalf64 {
    type Output = Self;
    fn one() -> Self::Output {
        Totalf64(1.0)
    }
}
