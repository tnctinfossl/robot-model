use super::*;
use serde_derive::*;
use std::ops::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Vec2Rad {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Vec2Rad {
    #[inline(always)]
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, theta: f32) -> Vec2Rad {
        Vec2Rad {
            x: x,
            y: y,
            theta: theta,
        }
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn from_vec2_rad(position: Vec2, theta: f32) -> Vec2Rad {
        Vec2Rad {
            x: position.x,
            y: position.y,
            theta: theta,
        }
    }

    //thetaを[0..2PI]の範囲に縮小する
    #[inline(always)]
    #[allow(dead_code)]
    pub fn shrink(self) -> Vec2Rad {
        let two_pi = 2.0 * std::f32::consts::PI;
        if self.theta < 0.0 {
            vec2rad(self.x, self.y, (self.theta % two_pi) + two_pi)
        } else {
            vec2rad(self.x, self.y, self.theta % two_pi)
        }
    }

    //前回の値をもとに、[0..2PI]から値域を拡大する
    #[inline(always)]
    #[allow(dead_code)]
    pub fn expend(self, last: &Vec2Rad) -> Self {
        let two_pi = 2.0 * std::f32::consts::PI;
        let theta_offset = (self.theta / two_pi).trunc() * two_pi;
        let theta_now = self.theta % two_pi;
        let theta_last = last.theta % two_pi;

        let theta = if (theta_now - theta_last).abs() < std::f32::consts::PI {
            (theta_now - theta_last) + theta_offset
        } else {
            if theta_now > theta_last {
                (theta_now - theta_last - two_pi) + theta_offset
            } else {
                (theta_now - theta_last + two_pi) + theta_offset
            }
        };
        vec2rad(self.x, self.y, theta)
    }

    //差分方程式を解く (p0-p1)/t
    #[inline(always)]
    #[allow(dead_code)]
    pub fn diff(period: f32, p0: Vec2Rad, p1: Vec2Rad) -> Vec2Rad {
        let pi = std::f32::consts::PI;
        let p0 = p0.shrink();
        let p1 = p1.shrink();
        //差分方程式を解く
        let dx = (p0.x - p1.x) / period;
        let dy = (p0.y - p1.y) / period;
        let dtheta = if (p0.theta - p1.theta).abs() < pi + std::f32::EPSILON {
            (p0.theta - p1.theta) / period
        } else {
            let two_pi = 2.0 * std::f32::consts::PI;
            if p0.theta > p1.theta {
                (p0.theta - p1.theta - two_pi) / period
            } else {
                (p0.theta - p1.theta + two_pi) / period
            }
        };
        vec2rad(dx, dy, dtheta)
    }
    //差分方程式を解く (p0-2p1-p2)/t^2
    #[inline(always)]
    #[allow(dead_code)]
    pub fn diff3(period: f32, p0: Vec2Rad, p1: Vec2Rad, p2: Vec2Rad) -> Vec2Rad {
        (Self::diff(period, p0, p1) - Self::diff(period, p1, p2)) / period
    }

    //差分方程式を解く (p0-2p1-p2)/t^2
    #[inline(always)]
    #[allow(dead_code)]
    pub fn diff4(period: f32, p0: Vec2Rad, p1: Vec2Rad, p2: Vec2Rad, p3: Vec2Rad) -> Vec2Rad {
        (Self::diff3(period, p0, p1, p2) - Self::diff3(period, p1, p2, p3)) / period
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn powi(&self, n: i32) -> Vec2Rad {
        Vec2Rad::new(self.x.powi(n), self.y.powi(n), self.theta.powi(n))
    }
    #[inline(always)]
    #[allow(dead_code)]
    pub fn powf(&self, x: f32) -> Vec2Rad {
        Vec2Rad::new(self.x.powf(x), self.y.powf(x), self.theta.powf(x))
    }
}
#[inline(always)]
#[allow(dead_code)]
pub fn vec2rad<T: Into<f32>>(x: T, y: T, theta: T) -> Vec2Rad {
    Vec2Rad::new(x.into(), y.into(), theta.into())
}

impl Add for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Vec2Rad::new(self.x + rhs.x, self.y + rhs.y, self.theta + rhs.theta)
    }
}

impl AddAssign for Vec2Rad {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.theta += rhs.theta;
    }
}

impl Sub for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Vec2Rad::new(self.x - rhs.x, self.y - rhs.y, self.theta - rhs.theta)
    }
}

impl SubAssign for Vec2Rad {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.theta -= rhs.theta;
    }
}

impl Neg for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Vec2Rad::new(-self.x, -self.y, -self.theta)
    }
}

impl Mul<f32> for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: f32) -> Self {
        Vec2Rad::new(self.x * rhs, self.y * rhs, self.theta * rhs)
    }
}

impl Mul<Vec2Rad> for f32 {
    type Output = Vec2Rad;
    #[inline(always)]
    fn mul(self, rhs: Vec2Rad) -> Vec2Rad {
        Vec2Rad::new(self * rhs.x, self * rhs.y, self * rhs.theta)
    }
}

impl MulAssign<f32> for Vec2Rad {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.theta *= rhs;
    }
}

impl Div<f32> for Vec2Rad {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: f32) -> Self {
        Vec2Rad::new(self.x / rhs, self.y / rhs, self.theta / rhs)
    }
}

impl DivAssign<f32> for Vec2Rad {
    #[inline(always)]
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.theta /= rhs;
    }
}

impl Index<usize> for Vec2Rad {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.theta,
            _ => &0.0,
        }
    }
}

impl From<Vec2Rad> for Vec2 {
    fn from(v: Vec2Rad) -> Vec2 {
        vec2(v.x, v.y)
    }
}

impl Dot<Vec2Rad> for Vec2Rad {
    type Output = f32;
    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.theta * rhs.theta
    }
}
