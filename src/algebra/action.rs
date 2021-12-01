pub trait Action {
    type FType;
    type UType;
    fn fold_id(&self) -> Self::FType;
    fn fold(&self, x: Self::FType, y: Self::FType) -> Self::FType;
    fn update_id(&self) -> Self::UType;
    fn update(&self, next: Self::UType, cur: Self::UType) -> Self::UType;
    fn convert(&self, x: Self::FType, y: Self::UType, l: usize) -> Self::FType;
}
