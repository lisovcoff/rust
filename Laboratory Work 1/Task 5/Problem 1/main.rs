fn main() {
    let chr: char = 'a';

    if chr.is_uppercase() {
        println!("Character '{}' is uppercase.", chr);
    } else if chr.is_lowercase() {
        println!("Character '{}' is lowercase.", chr);
    } else {
        println!("Character '{}' has no case.", chr);
    }
}
