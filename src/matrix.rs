pub struct Matrix{
    matrix: Vec<Vec<f64>>
}

impl Matrix{
    pub fn init_matrix(&mut self, input: Vec<Vec<f64>>){
        self.matrix = input;
    }

    pub fn print_matrix(&self){
        for row in &self.matrix{
            println!("{:?}", row);
        }
    }

    pub fn read_matrix(&self) -> &Vec<Vec<f64>> {
        return &self.matrix;
    }

    pub fn mut_matrix(&mut self) -> &mut Vec<Vec<f64>> {
        return &mut self.matrix;
    }
}
