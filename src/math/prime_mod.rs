//! 素数剰余用

use crate::math::primitive_root::primitive_root;

/// 素数剰余用
pub trait PrimeMod: Sized + Copy + Clone + PartialEq + Default {
    /// 素数
    const PRIME_NUM: u32;
    /// 原始根
    const PRIM_ROOT: u32;
}

/// 素数`P`での剰余
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, PartialOrd, Ord, Hash)]
pub struct Prime<const P: u32>;
impl<const P: u32> PrimeMod for Prime<P> {
    const PRIME_NUM: u32 = P;
    const PRIM_ROOT: u32 = primitive_root(P);
}
