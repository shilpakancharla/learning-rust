fn main() {
    let celsius_temperature = 23.0;
    let fahrenheit_temperature = celsius_to_fahrenheit(celsius_temperature);

    assert_eq!(fahrenheit_temperature, 73.4);
    println!("Test passed.")
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}
