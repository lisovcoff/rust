fn main() {
    let text = "abc_abc_abc";
    let result = text.replace('_', " ");

    println!("Original string: {}", text);
    println!("Result: {}", result);
}
