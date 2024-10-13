pub trait Action {
    type Output;
    type Lazy;
    fn fold_id(&self) -> Self::Output;
    fn fold(&self, left: Self::Output, right: Self::Output) -> Self::Output;
    fn update_id(&self) -> Self::Lazy;
    fn update(&self, next: Self::Lazy, cur: Self::Lazy) -> Self::Lazy;
    fn convert(&self, value: Self::Output, lazy: Self::Lazy, len: usize) -> Self::Output;
}
