pub trait Energy {
    type Output;
    fn energy(&self) -> Self::Output; //[J]
}

pub trait Simulate<T> {
    type Output;
    fn simulate(&self, time: T) -> Self::Output;
}
