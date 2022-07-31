pub trait Foldable<Idx> {
    type Output;
    fn fold(&self, range: Idx) -> Self::Output;
}

pub trait Foldable2D<Idx> {
    type Output;
    fn fold(&self, x_range: Idx, y_range: Idx) -> Self::Output;
}

pub trait Assignable<Idx> {
    type Value;
    fn assign(&mut self, i: Idx, value: Self::Value);
}

pub trait Updatable<Idx> {
    type Value;
    fn update(&mut self, i: Idx, value: Self::Value);
}

pub trait Indexable<Idx> {
    type Output;
    fn get(&self, i: Idx) -> Self::Output;
}
