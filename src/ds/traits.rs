pub trait Foldable<Idx> {
    type Output;
    fn fold(&self, range: Idx) -> Self::Output;
}

pub trait FoldableMut<Idx> {
    type Output;
    fn fold(&mut self, range: Idx) -> Self::Output;
}

pub trait Assignable<Idx> {
    type Value;
    fn assign(&mut self, i: Idx, value: Self::Value);
}

pub trait Updatable<Idx> {
    type Value;
    fn update(&mut self, i: Idx, value: Self::Value);
}

pub trait IndexableMut<Idx> {
    type Output;
    fn get(&mut self, i: Idx) -> Self::Output;
}
