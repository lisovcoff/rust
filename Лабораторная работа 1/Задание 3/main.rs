use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Введите первое число: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();

    input.clear();
    print!("Введите второе число: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    input.clear();
    print!("Введите операцию (+ или -): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let op = input.trim();

    match op {
        "+" => println!("Результат: {}", a + b),
        "-" => println!("Результат: {}", a - b),
        _ => println!("Ошибка: неизвестная операция"),
    }
}
