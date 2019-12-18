pub mod math;
pub mod physics;
pub mod robot;
pub mod traits;
pub use math::*;
pub use robot::*;
pub use traits::*;

fn main() {
    use math::pascal::*;
    let p = Pascal::new();
    println!("{:?}", p.delta(4));
}
