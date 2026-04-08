use std::io;

fn main() {

    println!("Enter the dimension of your matrix:");
    let mut dimension = String::new(); 
    io::stdin()
        .read_line(&mut dimension)
        .expect("Could not read input");

    let matrix = create_matrix(dimension);

    println!("{:?}", matrix);

}

fn create_matrix(dimension: String) -> Vec<Vec<i32>> {
    let n: usize = dimension
        .trim()
        .parse()
        .expect("number");

    let matrix = vec![vec![0; n]; n];

    return matrix;
}
