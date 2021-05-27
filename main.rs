fn main() {
    // Three variables with different data types
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    // Cast a as a 64 bit float
    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed.")
}