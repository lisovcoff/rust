fn main() {
    let num: i16 = 12000;

    let zeros = num.to_string()
        .chars()
        .rev()
        .take_while(|&c| c == '0')
        .count();

    println!("Число: {}", num);
    println!("Нули с конца: {}", "0".repeat(zeros));
}
