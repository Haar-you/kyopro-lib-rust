/// 両端の点を含む閉区間を扱う。
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ClosedInterval<T> {
    /// 閉区間の開始地点
    pub start: T,
    /// 閉区間の終了地点
    pub end: T,
}

impl<T> ClosedInterval<T> {
    /// [`ClosedInterval<T>`]を生成する
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

impl<T: Ord + PartialEq + Copy> ClosedInterval<T> {
    /// 2つの閉区間に共通部分があれば、統合した結果を`Ok`に包んで返す。
    ///
    /// 共通部分がなければ、`Err`に元の2つの閉区間を包んで返す。
    pub fn merge(self, other: Self) -> Result<Self, (Self, Self)> {
        if self.end < other.start || other.end < self.start {
            Err((self, other))
        } else {
            Ok(Self {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        type I = ClosedInterval<i64>;

        assert_eq!(
            I::new(0, 3).merge(I::new(4, 7)),
            Err((I::new(0, 3), I::new(4, 7)))
        );
        assert_eq!(I::new(0, 3).merge(I::new(3, 6)), Ok(I::new(0, 6)));
        assert_eq!(I::new(0, 3).merge(I::new(-2, 5)), Ok(I::new(-2, 5)));
        assert_eq!(I::new(0, 3).merge(I::new(-2, 2)), Ok(I::new(-2, 3)));
        assert_eq!(I::new(0, 3).merge(I::new(1, 2)), Ok(I::new(0, 3)));
    }
}
