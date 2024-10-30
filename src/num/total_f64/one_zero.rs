//! [`One`]と[`Zero`]を実装する。

use crate::num::one_zero::*;
pub use crate::num::total_f64::*;

impl Zero for Totalf64 {
    fn zero() -> Self {
        Totalf64(0.0)
    }
}

impl One for Totalf64 {
    fn one() -> Self {
        Totalf64(1.0)
    }
}
