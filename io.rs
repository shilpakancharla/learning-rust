use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number = buffer.trim().parse::<i32>();
    println!("number + 1 is {}", number + 1)
}