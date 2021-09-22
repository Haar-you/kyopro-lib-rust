pub trait Foldable<Idx> {
    type Value;
    fn fold(&self, range: Idx) -> Self::Value;
}

pub trait Assignable<Idx> {
    type Value;
    fn assign(&mut self, i: Idx, value: Self::Value);
}

pub trait Updatable<Idx> {
    type Value;
    fn update(&mut self, i: Idx, value: Self::Value);
}
