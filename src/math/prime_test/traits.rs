pub trait CheckPrime<T> {
    fn is_prime(&self, value: T) -> bool;
}
