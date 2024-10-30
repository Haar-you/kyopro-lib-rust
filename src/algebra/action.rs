pub trait Action {
    /// 範囲取得の型
    type Output;
    /// 範囲更新の型
    type Lazy;
    /// 範囲取得のモノイドの単位元
    fn fold_id(&self) -> Self::Output;
    /// 範囲取得の二項演算
    fn fold(&self, left: Self::Output, right: Self::Output) -> Self::Output;
    /// 範囲更新のモノイドの単位元
    fn update_id(&self) -> Self::Lazy;
    /// 範囲更新の二項演算
    fn update(&self, next: Self::Lazy, cur: Self::Lazy) -> Self::Lazy;
    /// 範囲更新を範囲取得に反映させる。
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output;
}
