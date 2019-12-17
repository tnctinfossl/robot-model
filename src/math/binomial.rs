use super::traits::*;
use std::ops::*;
//Pascalの三角形
//階上を求める
impl Factorial for u32 {
    type Output = Self;
    fn factorial(self) -> Self {
        if self < 2 {
            1
        } else {
            self * factorial(self - 1)
        }
    }
}

impl Factorial for f32 {
    type Output = Self;
    fn factorial(self) -> Self {
        if self < 2.0 {
            1.0
        } else {
            self * factorial(self - 1.0)
        }
    }
}

impl<T> Combination for T
where
    T: Factorial<Output = T> + Sub<T, Output = T> + Div<T, Output = T> + Mul<T, Output = T> + Copy,
{
    type Output = T;
    fn combination(self, m: T) -> T {
        self.factorial() / ((self - m).factorial() * m.factorial())
    }
}
