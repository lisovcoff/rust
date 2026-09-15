fn draw_matrix(n: usize, m: usize, sym: char) {
    for _ in 0..n {
        for _ in 0..m {
            print!("{} ", sym);
        }
        println!();
    }
}

fn draw_matrix_default() {
    draw_matrix(10, 10, '*');
}

fn main() {
    draw_matrix(3, 2, '*');
    println!();

    draw_matrix(5, 5, '#');
    println!();

    println!("Матрица по умолчанию:");
    draw_matrix_default();
}
