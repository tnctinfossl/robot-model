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
