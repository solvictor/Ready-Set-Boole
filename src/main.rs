mod operators;

use operators::multiplier;

fn main() {
    let a: u32 = u32::MAX;
    let b: u32 = u32::MAX;
    println!("a = {}, b = {}, a * b = {}", a, b, multiplier(a, b));
}
