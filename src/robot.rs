use super::*;
use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum RobotID {
    Blue(u32),
    Yellow(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Robot {
    //識別子
    pub id: RobotID,
    //座標系
    pub position: Vec2Rad,     //[m,m,rad]
    pub velocity: Vec2Rad,     //[m/s,m/s,rad/s]
    pub acceleration: Vec2Rad, //[m/s^2,m/s^2,rad/s^2]
    pub jerk: Vec2Rad,         //[m/s^3,m/s^3,rad/s^3]
    //質量
    pub mass: f32,   //[kg]
    pub moment: f32, //?
    //大きさ
    pub radius: f32, //[m]
}

impl Default for Robot {
    fn default() -> Self {
        Self {
            //識別子
            id: RobotID::Blue(0),
            //座標系
            position: Vec2Rad::default(),
            velocity: Vec2Rad::default(),
            acceleration: Vec2Rad::default(),
            jerk: Vec2Rad::default(),
            //質量等
            mass: 1.0,   //適当な値です
            moment: 1.0, //適当な値です
            //大きさ
            radius: 0.1, //適当な値です
        }
    }
}

impl Energy for Robot {
    type Output = f32;
    fn energy(&self) -> f32 {
        dot(
            vec2rad(self.mass, self.mass, self.moment),
            self.velocity.powi(2),
        ) / 2.0
    }
}

impl Simulate<f32> for Robot {
    type Output = Self;
    fn simulate(&self, time: f32) -> Self {
        //positionのみ変更する
        let position = self.position
            + self.velocity * time
            + self.acceleration / 2.0 * time.powi(2)
            + self.jerk / 6.0 * time.powi(3);
        let mut robot = self.clone();
        robot.position = position;
        robot
    }
}
