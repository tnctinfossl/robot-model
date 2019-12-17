use super::traits::*;
use super::vec2::*;
use std::ops::*;

//一回微分
pub struct DeltaOne {}
impl<T> Delta<T, (T, T)> for DeltaOne
where
    T: Sub<T, Output = T> + Div<T, Output = T>,
{
    type Output = T;
    fn delta(&self, period: T, (x0, x1): (T, T)) -> Self::Output {
        (x0 - x1) / period
    }
}
