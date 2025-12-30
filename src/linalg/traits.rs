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

pub trait Matrix {
    /// 行列の列数を返す。
    fn width(&self) -> usize;
    /// 行列の行数を返す。
    fn height(&self) -> usize;
    /// 行列が正方であるかどうかを判定する。
    fn is_square(&self) -> bool {
        self.width() == self.height()
    }
}
