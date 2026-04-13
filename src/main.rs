mod matrix;
mod operations;

use std::io::{self};
use matrix::Matrix;
use operations::{row_ops, echelon};

fn main() {
    let mut m = Matrix::new(vec![
        vec![1.0, 2.0, 3.0], 
        vec![2.0, 4.0, 6.0],
        vec![1.0, 1.0, 1.0]]);

    print_section("Printing Matrix");
    m.print_matrix();

    /*
    print_section("Swapping Rows 0 and 2");
    row_ops::swap_row(&mut m, 0, 2);
    m.print_matrix();
    */ 

    print_section("Echelon Form");
    echelon::row_echelon(&mut m);
    m.print_matrix();
}

fn print_section(title: &str) {
  println!("\n==================== {} ====================\n", title);
}
