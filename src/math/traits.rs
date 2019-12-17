pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}

#[inline(always)]
pub fn dot<T0, T1>(a: T0, b: T1) -> <T0 as Dot<T1>>::Output
where
    T0: Dot<T1>,
{
    a.dot(b)
}

pub trait Cross<Rhs = Self> {
    type Output;
    fn cross(self, rhs: Rhs) -> Self::Output;
}

#[inline(always)]
pub fn cross<T0, T1>(a: T0, b: T1) -> <T0 as Cross<T1>>::Output
where
    T0: Cross<T1>,
{
    a.cross(b)
}

pub trait Factorial {
    type Output;
    fn factorial(self) -> Self::Output;
}
#[inline(always)]
pub fn factorial<T>(x: T) -> <T as Factorial>::Output
where
    T: Factorial,
{
    x.factorial()
}

pub trait Combination {
    type Output;
    fn combination(self, n: Self) -> Self::Output;
}
#[inline(always)]
pub fn combination<T>(x: T, y: T) -> <T as Combination>::Output
where
    T: Combination,
{
    x.combination(y)
}
/*
//rustで定数をgenericにできないため。
pub trait GenericNumber {
    fn number() -> usize;
}*/
