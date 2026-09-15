fn main() {
    let text = "abc_abc_abc";
    let result = text.replace('_', " ");

    println!("Исходная строка: {}", text);
    println!("Результат: {}", result);
}
