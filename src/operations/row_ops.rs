use crate::matrix::Matrix;

pub fn swap_row(matrix: &mut Matrix, row_i: usize, row_j: usize){
    matrix.mut_matrix().swap(row_i, row_j);
}

pub fn scalar(matrix: &mut Matrix, row: usize, scalar: f64){
    for val in &mut matrix.mut_matrix()[row]{
        *val *= scalar;
    }
}
