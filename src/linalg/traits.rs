pub trait TryAdd<Rhs = Self> {
    type Output;
    fn try_add(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait TrySub<Rhs = Self> {
    type Output;
    fn try_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait TryMul<Rhs = Self> {
    type Output;
    fn try_mul(self, rhs: Rhs) -> Option<Self::Output>;
}
