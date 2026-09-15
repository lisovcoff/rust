fn main() {
    let chr: char = 'a';

    if chr.is_uppercase() {
        println!("Символ '{}' в верхнем регистре.", chr);
    } else if chr.is_lowercase() {
        println!("Символ '{}' в нижнем регистре.", chr);
    } else {
        println!("Символ '{}' не имеет регистра.", chr);
    }
}
