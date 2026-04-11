mod matrix;

use std::io::{self};
use matrix::Matrix;

fn main() {
    let m = Matrix::new(vec![
        vec![1.0, 0.0], 
        vec![0.0, 1.0]]);

    m.print_matrix();
}

/*
fn row_echelon(mut matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
  let row_count = matrix.len();
  let col_count = matrix[0].len();

  let mut pivot_row = 0;

  for pivot_col in 0..col_count {
    if pivot_row >= row_count {
      break;
    }

    // finding pivot
    let mut max_row = pivot_row;
    while max_row < row_count && matrix[max_row][pivot_col] == 0.0 {
      max_row += 1;
    }

    if max_row == row_count {
      continue;
    }

    // swap rows
    if max_row != pivot_row {
      matrix.swap(pivot_row, max_row);
    }

    // make pivot = 1
    let pivot = matrix[pivot_row][pivot_col];
    for col in pivot_col..col_count {
      matrix[pivot_row][col] /= pivot;
    }

    // eliminate below
    for row in pivot_row + 1..row_count {
      let factor = matrix[row][pivot_col];

      for col in pivot_col..col_count {
        matrix[row][col] -= factor * matrix[pivot_row][col];
      }
    }

    pivot_row += 1;
  }

  return matrix;
}

fn define_values(mut matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {

    let mut row_counter = 0;
    for row in 0..matrix.len() {
        loop{
            println!("Enter values for the row {row_counter} seperated by spaces:\n");
        
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Invalid values");

            let values: Vec<f64> = input
                .split_whitespace()
                .map(|x| x.parse::<f64>().expect("Enter a valid input"))
                .collect();

            if values.len() != matrix[row].len() {
                let length = matrix[row].len();
                println!("Please enter the correct number of values ({length})\n");
                continue;
            }
        
            matrix[row] = values;
            row_counter += 1;
            println!("");
            break;
        }
    }

    return matrix;
}
*/
