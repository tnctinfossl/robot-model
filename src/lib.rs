#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        //assert_eq!(2 + 2, 4);
    }
}

pub mod math;
pub use math::*;
pub mod object;
pub use traits::*;
