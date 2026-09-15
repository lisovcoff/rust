use std::io;

fn main() {
    let arr = [10, 3, 25, 7, 40, 11, 15, 2];

    println!("Numbers divisible by 5:");
    for &num in &arr {
        if num % 5 == 0 {
            println!("{}", num);
        }
    }

    let mut input = String::new();
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut input).unwrap();
}
