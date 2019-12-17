#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod math;
pub mod physics;
pub mod robot;
pub mod traits;
pub use math::*;
pub use robot::*;
pub use traits::*;
