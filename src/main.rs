use rust_linear_algebra::Vector;

fn main() {
    let a = Vector::from_array([1.0, 0.5]);
    let b = Vector::from_array([0.5, 1.5]);
    println!("{a} + {b} = {}", a+b);
    println!("Hello, world!");
}
