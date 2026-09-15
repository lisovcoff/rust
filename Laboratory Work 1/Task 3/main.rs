use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();

    input.clear();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    input.clear();
    print!("Enter the operation (+ or -): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let op = input.trim();

    match op {
        "+" => println!("Result: {}", a + b),
        "-" => println!("Result: {}", a - b),
        _ => println!("Error: unknown operation"),
    }
}
