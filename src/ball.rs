use super::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ball {
    //座標系
    pub position: Vec2,     //[m,m]
    pub velocity: Vec2,     //[m/s,m/s]
    pub acceleration: Vec2, //[m/s^2,m/s^2]
    pub jerk: Vec2,         //[m/s^3,m/s^3]
    //質量
    pub mass: f32, //[kg]
    //大きさ
    pub radius: f32, //[m]
}

impl Default for Ball {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            velocity: Vec2::default(),
            acceleration: Vec2::default(),
            jerk: Vec2::default(),
            mass: 0.01,
            radius: 0.02,
        }
    }
}

impl Simulate<f32> for Ball {
    type Output = Self;
    fn simulate(&self, time: f32) -> Self {
        //positionのみ変更する
        let position = self.position
            + self.velocity * time
            + self.acceleration / 2.0 * time.powi(2)
            + self.jerk / 6.0 * time.powi(3);
        let mut ball = self.clone();
        ball.position = position;
        ball
    }
}

impl Energy for Robot {
    type Output = f32;
    fn energy(&self) -> f32 {
        dot(vec2(self.mass, self.mass), self.velocity.powi(2)) / 2.0
    }
}
