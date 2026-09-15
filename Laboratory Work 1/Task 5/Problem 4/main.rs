fn main() {
    let arr = [1, 2, 3, 0, 4, 5];

    if let Some(pos) = arr.iter().position(|&x| x == 0) {
        let before = &arr[..pos];
        let after = &arr[pos + 1..];

        println!("Before 0: {:?}", before);
        println!("After 0: {:?}", after);
    } else {
        println!("Element 0 was not found.");
    }
}
