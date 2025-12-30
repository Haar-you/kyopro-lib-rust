//! トレイト

/// 失敗可能性のある足し算を表す。
pub trait TryAdd<Rhs = Self> {
    /// 返り値の型
    type Output;
    /// 失敗可能性のある足し算を行う。
    fn try_add(self, rhs: Rhs) -> Option<Self::Output>;
}

/// 失敗可能性のある引き算を表す。
pub trait TrySub<Rhs = Self> {
    /// 返り値の型
    type Output;
    /// 失敗可能性のある引き算を行う。
    fn try_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

/// 失敗可能性のある掛け算を表す。
pub trait TryMul<Rhs = Self> {
    /// 返り値の型
    type Output;
    /// 失敗可能性のある掛け算を行う。
    fn try_mul(self, rhs: Rhs) -> Option<Self::Output>;
}

/// 行列
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

/// 行列の転置
pub trait MatrixTranspose {
    /// 転置行列の型
    type Output;
    /// 転置した行列を返す。
    fn transpose(self) -> Self::Output;
}
