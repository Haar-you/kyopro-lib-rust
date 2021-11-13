pub trait Action<T, U> {
    fn fold_id(&self) -> T;
    fn fold(&self, x: T, y: T) -> T;
    fn update_id(&self) -> U;
    fn update(&self, next: U, cur: U) -> U;
    fn convert(&self, x: T, y: U, l: usize) -> T;
}
